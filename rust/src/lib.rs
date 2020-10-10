#[doc(hidden)]
pub extern crate libc;

#[macro_export]
macro_rules! plugin {
    (
        $name:literal version $version:literal: 
    ) => {
        #[no_mangle]
        pub fn plugin_main() -> *const $crate::libc::c_char {
            (concat!($name, "\0")).as_ptr() as *const $crate::libc::c_char
        }

        #[no_mangle]
        pub fn plugin_version() -> *const $crate::libc::c_char {
            (concat!($version, "\0")).as_ptr() as *const $crate::libc::c_char
        }

        #[no_mangle]
        pub fn plugin_ffi_version() -> *const $crate::libc::c_char {
            "0.1.0".as_ptr() as *const $crate::libc::c_char
        }

        #[no_mangle]
        pub fn plugin_entry_point() {

        }
    };
}
