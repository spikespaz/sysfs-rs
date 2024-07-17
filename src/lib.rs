#![doc = include_str!("../README.md")]
//!
//! ## Calling convention for [`api`]
//!
//! ```rust
//! use sysfs::api::psu::power_supply::{self, ChargeBehavior};
//!
//! // Read
//! let _ = power_supply::charge_behaviour("BAT0").expect("a battery in your computer");
//! // Write
//! power_supply::set_charge_behavior("BAT0", ChargeBehavior::Auto)).expect("super user permissions");
//! ```

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
