#[doc(hidden)]
pub mod cpu; // Can I get rid of the doc hidden?

pub mod lib {
    pub use sysfs_lib::*;
    pub use sysfs_macros::*;
}

pub mod api {
    pub use super::cpu;
}
