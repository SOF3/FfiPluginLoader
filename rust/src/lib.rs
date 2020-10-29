use std::rc::Rc;

use ffi::size_t;
use libc::c_char;

#[doc(hidden)]
pub extern crate libc;
#[doc(hidden)]
pub extern crate static_assertions;
#[doc(hidden)]
pub use once_cell::sync::OnceCell;

// #[doc(hidden)]
#[allow(non_camel_case_types, non_upper_case_globals)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bind.rs"));
}

#[macro_export]
macro_rules! plugin {
    (
        $name:literal version $version:literal: $impl:ty
    ) => {
        $crate::static_assertions::assert_impl_all!($impl: $crate::Plugin);

        static __POCKETMINE_PLUGIN: $crate::OnceCell<$impl> = $crate::OnceCell::new();

        /// FFI function
        #[no_mangle]
        pub fn plugin_main() -> *const $crate::libc::c_char {
            (concat!($name, "\0")).as_ptr() as *const $crate::libc::c_char
        }

        /// FFI function
        #[no_mangle]
        pub fn plugin_version() -> *const $crate::libc::c_char {
            (concat!($version, "\0")).as_ptr() as *const $crate::libc::c_char
        }

        /// FFI function
        #[no_mangle]
        pub fn plugin_ffi_version() -> *const $crate::libc::c_char {
            "0.1.0".as_ptr() as *const $crate::libc::c_char
        }

        /// FFI function
        ///
        /// # Safety
        /// See manifest.h
        #[no_mangle]
        pub unsafe fn plugin_entry_point(api: $crate::ffi::api) -> bool {
            let plugin = match <$impl as $crate::Plugin>::init($crate::Api::from(api)) {
                Ok(plugin) => plugin,
                Err(err) => {
                    unsafe {
                        // This function is always called from the main thread
                        $crate::set_errstr(err);
                    }
                    return false;
                }
            };
            let _ = __POCKETMINE_PLUGIN.set(plugin); // plugin_entry_point should only be called once
            true
        }
    };
}

/// Sets the error string to use
///
/// # Safety
/// This function must be called on the main thread.
pub unsafe fn set_errstr(err: impl ToString) {
    let mut err = err.to_string();
    let mut bytes = err.as_bytes_mut(); // safety: we won't use the original `err` again
    if bytes.len() > 4095 {
        bytes = &mut bytes[..4095];
        for byte in bytes.iter_mut().skip(4092) {
            *byte = b'.';
        }
    }

    let errstr = ffi::errstr.as_mut_ptr() as *mut u8; // safety: only called from main thread
    let errstr = std::slice::from_raw_parts_mut(errstr, 4096); // safety: i8 and u8 have same size
    errstr.copy_from_slice(bytes);
    errstr[bytes.len()] = b'\0';
}

pub struct Api {
    ffi: Rc<ffi::api>,
}

impl From<ffi::api> for Api {
    fn from(ffi: ffi::api) -> Self {
        Self { ffi: Rc::new(ffi) }
    }
}

pub struct CommandApi<'t>(&'t Api);

impl<'t> CommandApi<'t> {
    pub fn register(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        usage: impl Into<String>,
        aliases: &[&str],
        permission: impl Into<String>,
    ) {
        let mut name = name.into();
        name.push('\0');
        let mut description = description.into();
        description.push('\0');
        let mut usage = usage.into();
        usage.push('\0');
        let mut permission = permission.into();
        permission.push('\0');

        let mut aliases_c = String::new();
        for alias in aliases {
            aliases_c += alias;
            aliases_c.push('\0');
        }
        let aliases_list = ffi::string_list {
            ptr: aliases_c.as_ptr() as *const c_char,
            size: aliases.len() as size_t,
        };

        let f = self.0.ffi.command.register_command.unwrap();
        unsafe {
            f(
                name.as_ptr() as *const c_char,
                description.as_ptr() as *const c_char,
                usage.as_ptr() as *const c_char,
                aliases_list,
                permission.as_ptr() as *const c_char,
            );
        }
    }
}

pub trait Plugin: Sized {
    fn init(api: Api) -> Result<Self, anyhow::Error>;
}
