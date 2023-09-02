Generate smart pointers for [windows](https://crates.io/crates/windows) raw handles with ergonomic APIs.

This crate doesn't offer pre-defined smart pointers. Instead, it provides a single [`safe_handle!`] macro for generation:

### Simple Smart Pointer, calling an unsafe Function on `Drop`
```rust
use windows_safe_handle::safe_handle;
use windows::Win32::Foundation::{HANDLE, CloseHandle};

safe_handle!(pub Handle(HANDLE), CloseHandle);
```
If you do not need to export the `Handle` type, simply omit the `pub` keyword.

### Smart Pointer with additional `Drop` logic
You can use a closure-based syntax:
```rust
use windows_safe_handle::safe_handle;
use windows::Win32::Foundation::{HANDLE, CloseHandle};

safe_handle!(pub Handle(HANDLE), |h| {
    // Place your code here
    unsafe { CloseHandle(h) }
});
```
Note that in this case you have to explicitly use `unsafe` block.

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
//                              ^^^^^^^^^^ it's the trick
let mut bitmap = create_new_bitmap();

# let hdc = windows::Win32::Graphics::Gdi::HDC::default();
# unsafe {
SelectObject(hdc, &bitmap); // works as expected
# }
```

## Example
Refer to `tests/bcrypt_hash.rs` to see how to safely wrap [Windows Cryptography Next Generation (CNG) APIs](https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal) for calculating MD5 hashes.
