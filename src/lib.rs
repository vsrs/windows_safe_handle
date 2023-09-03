#![doc = include_str!("../README.md")]


/*!
 ## Strict handles
All [windows](https://crates.io/crates/windows) handle types are defined to be mutually exclusive; you will not be able to pass an [`HWND`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HWND.html) where an [`HDC`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HDC.html) type argument is required. However, there are situations when you need to pass, for example,
[`HBITMAP`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HBITMAP.html) or 
[`HPEN`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HPEN.html) to a function expecting
[`HGDIOBJ`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HGDIOBJ.html) (like [`SelectObject`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/fn.SelectObject.html)). To do this, you have to specify the additional handle type:
```rust
# use windows_safe_handle::safe_handle;
# use windows::Win32::Graphics::Gdi::{HGDIOBJ, HBITMAP, DeleteObject, SelectObject};
# fn create_new_bitmap() -> Handle { Handle::default() }
safe_handle!(pub Handle(HBITMAP as HGDIOBJ), DeleteObject);
//                              ^^^^^^^^^^ that's the trick
let mut bitmap = create_new_bitmap();

# let hdc = windows::Win32::Graphics::Gdi::HDC::default();
# unsafe {
SelectObject(hdc, &bitmap); // works as expected
# }
```
*/

/// See [crate root documentation](crate).
#[macro_export]
macro_rules! safe_handle {
    ($vis:vis $type:ident($inner:ty), |$val:ident| $deleter: block) => {
        #[repr(transparent)]
        $vis struct $type($inner);

        impl Default for $type {
            #[inline(always)]
            fn default() -> Self {
                Self(<$inner>::default())
            }
        }

        impl Drop for $type {
            fn drop(&mut self) {
                self.close();
            }
        }

        #[allow(dead_code)]
        impl $type {
            /// # Safety
            /// 'handle' should be valid
            #[inline(always)]
            pub unsafe fn attach(handle: $inner) -> Self {
                Self(handle)
            }

            /// # Safety
            /// do not free the handle
            #[inline(always)]
            pub unsafe fn as_mut(&mut self) -> &mut $inner {
                &mut self.0
            }

            #[inline(always)]
            pub fn is_valid(&self) -> bool {
                !self.is_invalid()
            }

            #[inline(always)]
            pub fn is_invalid(&self) -> bool {
                self.0.is_invalid()
            }

            pub fn close(&mut self) -> bool {
                if self.is_valid() {
                    let $val = core::mem::take(&mut self.0);
                    let _res = $deleter;
                    true
                } else {
                    false
                }
            }
        }

        impl windows::core::IntoParam<$inner> for & $type {
            fn into_param(self) -> windows::core::Param<$inner> {
                windows::core::Param::Borrowed(windows::core::Type::abi(&self.0))
            }
        }

        impl windows::core::IntoParam<$inner> for &mut $type {
            fn into_param(self) -> windows::core::Param<$inner> {
                windows::core::Param::Borrowed(windows::core::Type::abi(&self.0))
            }
        }
    };

    ($vis:vis $type:ident($inner:ty), $deleter: path) => {
        $crate::safe_handle!($vis $type($inner), |x| { unsafe { $deleter(x) } });
    };

    ($vis:vis $type:ident($inner:ty as $into:ty), |$val:ident| $deleter: block) => {
        $crate::safe_handle!($vis $type($inner), |$val| $deleter);

        impl windows::core::IntoParam<$into> for & $type {
            fn into_param(self) -> windows::core::Param<$into> {
                self.0.into_param()
            }
        }
        
        impl windows::core::IntoParam<$into> for &mut $type {
            fn into_param(self) -> windows::core::Param<$into> {
                self.0.into_param()
            }
        }
    };

    ($vis:vis $type:ident($inner:ty as $into:ty), $deleter: path) => {
        $crate::safe_handle!($vis $type($inner as $into), |x| { unsafe { $deleter(x) } });
    };
}
