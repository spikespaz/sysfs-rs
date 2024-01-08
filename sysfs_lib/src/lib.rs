// <https://www.kernel.org/doc/html/latest/filesystems/sysfs.html>
// <https://www.kernel.org/doc/html/latest/admin-guide/sysfs-rules.html>
//
// If you see unchecked string functions being called,
// it's because *sysfs* is guaranteed to be ASCII (where we expect text).

// The maximum number of bytes that can be read from any given
// *sysfs* attribute. Generally there should be nothing larger than this.

use std::fs::OpenOptions;
use std::io::{ErrorKind, Read as _, Write as _};

pub type Result<T> = std::result::Result<T, SysfsError>;

#[derive(Debug, thiserror::Error)]
pub enum SysfsError {
    /// Kernel documentation says that if you get os error 2 that
    /// means a feature is unavailable.
    #[error("the requested sysfs attribute does not exist")]
    MissingAttribute,
    /// Sometimes attributes are unsupported on a platform.
    #[error("the requested sysfs attribute is not supported on this platform")]
    UnsupportedAttribute,

    #[error("encountered IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub const SYSFS_MAX_ATTR_BYTES: usize = 1024;

/// # Safety
///
/// This function makes an assumption that the contents of the file at
/// `file_path` is valid UTF-8 (in fact, it is *expected* to be ASCII).
///
/// This function is only safe if you are using Linux and have provided a
/// correct path to the attribute in *sysfs*. No validation is performed.
///
/// The function that you pass for `parse_ok` will also likely require usage of
/// `.unwrap()` when parsing the file content as an attribute value.
///
/// It is undefined behavior to use this function with file paths not exposed
/// through *sysfs*.
pub unsafe fn sysfs_read<T>(file_path: &str, parse_ok: fn(&str) -> T) -> Result<T> {
    let mut buf = [0; SYSFS_MAX_ATTR_BYTES];
    let result = OpenOptions::new()
        .read(true)
        .open(file_path)
        .and_then(|mut f| {
            let bytes_read = f.read(&mut buf)?;
            // SAFETY: Linux guarantees that all of *sysfs* is valid ASCII.
            let buf = unsafe { std::str::from_utf8_unchecked(&buf[..bytes_read]) };
            let buf = buf.trim_end();
            Ok(buf)
        });

    match result {
        Ok("<unsupported>") => Err(SysfsError::UnsupportedAttribute),
        Ok(text) => Ok(parse_ok(text)),
        Err(e) if e.kind() == ErrorKind::NotFound => Err(SysfsError::MissingAttribute),
        Err(e) => Err(SysfsError::from(e)),
    }
}

/// This is a low-level function which opens a file only if it already exists,
/// writes a string, and wraps error handling. It does not validate, so ensure
/// that your input is appropriate for the *sysfs* attribute in question.
pub fn sysfs_write(file_path: &str, value: impl AsRef<str>) -> Result<()> {
    OpenOptions::new()
        .read(false)
        .write(true)
        .create(false)
        .open(file_path)
        .and_then(|mut f| write!(f, "{}", value.as_ref()))
        .map_err(|e| {
            if e.kind() == ErrorKind::NotFound {
                SysfsError::MissingAttribute
            } else {
                SysfsError::from(e)
            }
        })
}

pub fn parse_selected(text: &str) -> Option<&str> {
    let (open, close) = (text.find('[')?, text.find(']')?);
    text.get(open + 1..close)
}
