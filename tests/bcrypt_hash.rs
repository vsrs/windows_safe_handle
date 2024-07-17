use windows::{
    core::{w, Result, PCWSTR},
    Win32::Security::Cryptography::*,
};

use windows_safe_handle::safe_handle;

safe_handle!(pub AlgProvider(BCRYPT_ALG_HANDLE as BCRYPT_HANDLE), |x| {
    unsafe { BCryptCloseAlgorithmProvider(x, 0) }
});

impl AlgProvider {
    pub const MD5: PCWSTR = w!("MD5");

    pub fn open(id: PCWSTR) -> Result<Self> {
        let mut handle = Self::default();
        let result = unsafe {
            BCryptOpenAlgorithmProvider(
                handle.as_mut(),
                id,
                PCWSTR::null(),
                BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
            )
        };

        result.to_hresult().map(|| handle)
    }

    pub fn create_hash(&self) -> Result<Hash> {
        let mut handle = Hash::default();
        let result =
            unsafe { BCryptCreateHash(self.0, handle.as_mut(), None, None, 0) };

        result.to_hresult().map(|| handle)
    }
}

safe_handle!(pub Hash(BCRYPT_HASH_HANDLE as BCRYPT_HANDLE), BCryptDestroyHash);

impl Hash {
    pub fn hash_data(&mut self, bytes: &[u8]) -> Result<()> {
        let result = unsafe { BCryptHashData(self.0, bytes, 0) };
        result.ok()
    }

    pub fn hash_length(&self) -> Result<u32> {
        let mut length = u32::default();
        let ptr = &mut length as *mut _ as *mut u8;
        let mut buffer = unsafe {
            std::slice::from_raw_parts_mut(ptr, std::mem::size_of::<u32>())
        };
        let mut dummy = 0_u32;
        let result = unsafe {
            BCryptGetProperty(
                self,
                BCRYPT_HASH_LENGTH,
                Some(&mut buffer),
                &mut dummy,
                0,
            )
        };

        result.to_hresult().map(|| length)
    }

    pub fn finish(self) -> Result<Vec<u8>> {
        let size = self.hash_length()?;
        let mut buffer = Vec::with_capacity(size as usize);

        #[allow(clippy::uninit_vec)]
        unsafe {
            // Ideally, we should pass `buffer.spare_capacity_mut()` to `BCryptFinishHash`.
            // However, it expects `&mut [u8]`, and since it doesn't read the provided buffer,
            // `set_len` is an easier option.
            buffer.set_len(buffer.capacity());
            self.finish_into(&mut buffer)?;
        }

        Ok(buffer)
    }

    /// # Safety
    /// hash_buffer length must exactly match the size of the hash or MAC value.
    pub unsafe fn finish_into(self, hash_buffer: &mut [u8]) -> Result<()> {
        let result = unsafe { BCryptFinishHash(self.0, hash_buffer, 0) };
        result.ok()
    }
}

// "abc", "90015098 3cd24fb0 d6963f7d 28e17f72"
const ABC_MD5: &[u8] = &[
    0x90, 0x1, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0, 0xd6, 0x96, 0x3f, 0x7d,
    0x28, 0xe1, 0x7f, 0x72,
];

#[test]
fn hash_test() {
    let alg_provider = AlgProvider::open(AlgProvider::MD5).unwrap();
    let mut hasher = alg_provider.create_hash().unwrap();
    hasher.hash_data(b"abc").unwrap();

    let hash = hasher.finish().unwrap();

    assert_eq!(hash, ABC_MD5);
}

#[test]
fn splitted_hash_test() {
    let alg_provider = AlgProvider::open(AlgProvider::MD5).unwrap();
    let mut hasher = alg_provider.create_hash().unwrap();
    hasher.hash_data(b"a").unwrap();
    hasher.hash_data(b"b").unwrap();
    hasher.hash_data(b"c").unwrap();

    let hash = hasher.finish().unwrap();
    assert_eq!(hash, ABC_MD5);
}

#[test]
fn splitted_hash_test2() {
    let alg_provider = AlgProvider::open(AlgProvider::MD5).unwrap();
    let mut hasher = alg_provider.create_hash().unwrap();
    hasher.hash_data(b"ab").unwrap();
    hasher.hash_data(b"c").unwrap();

    let hash = hasher.finish().unwrap();
    assert_eq!(hash, ABC_MD5);
}

#[test]
fn close_test() {
    let mut alg_provider = AlgProvider::open(AlgProvider::MD5).unwrap();
    assert!(alg_provider.is_valid());

    alg_provider.close();
    assert!(!alg_provider.is_valid());
}
