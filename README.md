# sysfs-rs

> [!IMPORTANT]
> Please note that this crate is very incomplete, and will not be considered stable for quite some time. The public API is subject to change, drastically.
> As more `sysfs` attributes are wrapped and documented, it may be desireable to change the organizational structure to offer a more homogeneous and consistent developer experience.

## What is this?

This crate embodies three core goals:

1. Provide safe functions and types for (eventually) all of Linux's `sysfs` attributes in a well-organized hierarchy.

2. Document each `sysfs` attribute, as well as the types of data being read from and written to these attributes. Information about `sysfs` is spread around everywhere, from comments in source code to discussion threads in mailing lists. This crate aims to aggregate that knowledge, and make it easily digestible.

3. Provide attribute macro helpers and transformation functions to generate accessors for any given `sysfs` attribute. Since `sysfs` is quite large and individual attributes are difficult to discover and get right, this crate's internal macros are exported for other developers, so that they may define attributes and modules which are missing in this crate.
    - Please contribute attribute definitions!

See the [Wikipedia page][1] and the [Kernel documentation][2] for more information about Linux's `sysfs`.

[1]: https://en.wikipedia.org/wiki/Sysfs
[2]: https://www.kernel.org/doc/html/latest/filesystems/sysfs.html

## How do I use it?

First, there are some modules defined for `sysfs` directories whose attributes have already been wrapped. Browse the documentation in the crate's [`sysfs::api`] module to see if it has what you need. There are getter functions, for example [`sysfs::api::psu::power_supply::capacity`], which reads the charge level of the battery as an `f32` percentage value (`0.00 .. 1.00`). Then there are setters, which serialize and write typed data to a `sysfs` attribute file, such as [`sysfs::api::psu::power_supply::set_charge_behavior`].

[`sysfs::api::psu::power_supply::capacity`]: https://docs.rs/sysfs/latest/sysfs/api/psu/power_supply/fn.capacity.html
[`sysfs::api::psu::power_supply::set_charge_behavior`]: https://docs.rs/sysfs/latest/sysfs/api/psu/power_supply/fn.set_charge_behaviour.html

Second, if there is no wrapper already defined in this crate's [`sysfs::api`] module, feel free to implement your own! See the documentation on items in [`sysfs::lib`] and the source code of existing modules (such as [`sysfs::api::cpu`]) for an idea of how to get started. Also, contributions to the `api` module are very much welcome and appreciated!

[`sysfs::lib`]: https://docs.rs/sysfs/latest/sysfs/lib/index.html
[`sysfs::api`]: https://docs.rs/sysfs/latest/sysfs/api/index.html
[`sysfs::api::cpu`]: https://docs.rs/sysfs/latest/src/sysfs/api/cpu.rs.html
