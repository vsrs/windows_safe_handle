# `windows_safe_handle`

Generate smart pointers for [windows](https://crates.io/crates/windows) raw handles with ergonomic APIs.

## `safe_handle!`

This crate doesn't offer pre-defined smart pointers. Instead, it provides a single `safe_handle!` macro for generation:

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

## Versions

- `v0.1.*`: Compatible with `windows` `v0.48.0`.
- `v0.2.*`: Compatible with `windows` `v0.58.0`.

## Example

Refer to [`tests/bcrypt_hash.rs`](https://github.com/vsrs/windows_safe_handle/blob/main/tests/bcrypt_hash.rs) to see how to safely wrap [Windows Cryptography Next Generation (CNG) APIs](https://learn.microsoft.com/en-us/windows/win32/seccng/cng-portal) for calculating MD5 hashes.
