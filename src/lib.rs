#![doc = include_str!("../README.md")]
/*!```*/
#![doc = include_str!("../tests/bcrypt_hash.rs")]
/*```*/


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
                if !self.0.is_invalid() {
                    let $val = self.0;
                    let _res = $deleter;
                }
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
