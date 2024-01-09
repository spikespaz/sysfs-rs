pub mod lib {
    pub use sysfs_lib::*;
    pub use sysfs_macros::*;
}

pub mod api {
    pub mod cpu;
    pub mod psu;
}

/// Stylistic:
///
/// Intended to be used as `sysfs::Error`, not imported.
/// If a  consumer module uses more items from `sysfs::lib`,
/// it modules should `use sysfs::lib::Error`, not `sysfs::Error`.
pub use lib::Error;
/// Stylistic: Same rules as `sysfs::Error`.
pub use lib::Result;
