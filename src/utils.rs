// <https://www.kernel.org/doc/html/latest/filesystems/sysfs.html>
// <https://www.kernel.org/doc/html/latest/admin-guide/sysfs-rules.html>
//
// If you see unchecked string functions being called,
// it's because *sysfs* is guaranteed to be ASCII (where we expect text).

// The maximum number of bytes that can be read from any given
// *sysfs* attribute. Generally there should be nothing larger than this.

use std::fmt;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read as _, Write as _};

use super::{Result, SysfsError};

pub const SYSFS_MAX_ATTR_BYTES: usize = 1024;

/// UNSAFE: This function assumes makes an assumption that the contents of
/// the file at `file_path` is valid UTF-8 (in fact, it is expected to be
/// ASCII). It is undefined behavior to use this function with file paths not
/// exposed through *sysfs*.
pub(crate) unsafe fn sysfs_read<T>(file_path: &str, parse_ok: fn(&str) -> T) -> Result<T> {
    let mut buf = [0; SYSFS_MAX_ATTR_BYTES];
    let result = OpenOptions::new()
        .read(true)
        .open(file_path)
        .and_then(|mut f| {
            let bytes_read = f.read(&mut buf)?;
            // SAFETY: Linux guarantees that all of *sysfs* is valid ASCII.
            let buf = unsafe { std::str::from_utf8_unchecked(&buf[..bytes_read]) };
            let buf = buf.trim_end_matches('\n');
            Ok(buf)
        });

    match result {
        Ok("<unsupported>") => Err(SysfsError::UnsupportedAttribute),
        Ok(text) => Ok(parse_ok(text)),
        Err(e) if e.kind() == ErrorKind::NotFound => Err(SysfsError::MissingAttribute),
        Err(e) => Err(SysfsError::from(e)),
    }
}

/// This is a low-level function which opens a file and writes
/// [`std::fmt::Arguments`] and wraps error handling. It does not validate, so
/// ensure that your input is appropriate for the *sysfs* attribute in question.
pub(crate) fn sysfs_write(file_path: &str, fmt_args: fmt::Arguments<'_>) -> Result<()> {
    OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .open(file_path)
        .and_then(|mut f| f.write_fmt(fmt_args))
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                SysfsError::MissingAttribute
            } else {
                SysfsError::from(e)
            }
        })
}

/// UNSAFE: Path templates to `sysfs` attributes are expected to be hard-coded
/// and validated before end-user runtime.
macro_rules! impl_sysfs_attrs {
    () => {};
    // Read-only
    (
        $(#[$attr_meta:meta])*
        $vis:vis sysfs_attr $attr_name:ident ($($arg_ident:ident : $arg_ty:ty),*)
        in $sysfs_dir:literal {
            $(#[$read_meta:meta])*
            read: $parse_ok:expr => $read_ty:ty,
        }

        $($tail:tt)*
    ) => {
        $(#[$attr_meta])*
        #[doc = ""]
        $(#[$read_meta])*
        $vis fn $attr_name ($($arg_ident : $arg_ty),*) -> $crate::Result<$read_ty> {
            let file_path = format!("{}/{}", format_args!($sysfs_dir), stringify!($attr_name));
            unsafe {
                $crate::sysfs_read::< $read_ty >(&file_path, $parse_ok)
            }
        }

        $crate::impl_sysfs_attrs! { $($tail)* }
    };
    // Read-write
    (
        $(#[$attr_meta:meta])*
        $vis:vis sysfs_attr $attr_name:ident ($($arg_ident:ident : $arg_ty:ty),*)
        in $sysfs_dir:literal {
            $(#[$read_meta:meta])*
            read: $parse_ok:expr => $read_ty:ty,
            $(#[$write_meta:meta])*
            write: | $val_ident:ident : $val_ty:ty | -> $fmt_args:expr,
        }

        $($tail:tt)*
    ) => {
        // Getter
        $crate::impl_sysfs_attrs! {
            $(#[$attr_meta])*
            $vis sysfs_attr $attr_name ($($arg_ident : $arg_ty),*)
            in $sysfs_dir {
                $(#[$read_meta])*
                read: $parse_ok => $read_ty,
            }
        }

        // Setter
        paste::paste! {
            $(#[$attr_meta])*
            #[doc = ""]
            $(#[$write_meta])*
            $vis fn [< set_ $attr_name >] ($($arg_ident: $arg_ty,)* $val_ident: $val_ty) -> $crate::Result<()> {
                let file_path = format!("{}/{}", format_args!($sysfs_dir), stringify!($attr_name));
                $crate::sysfs_write(&file_path, $fmt_args)
            }
        }

        $crate::impl_sysfs_attrs! { $($tail)* }
    };
}

pub(crate) use impl_sysfs_attrs;
