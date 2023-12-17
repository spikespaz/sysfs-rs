//! <https://www.kernel.org/doc/html/latest/admin-guide/pm/cpufreq.html?highlight=schedutil#policy-interface-in-sysfs>
use std::os::unix::ffi::OsStrExt;

use super::utils::impl_sysfs_attrs;
use super::Result;

pub static SYSFS_DIR: &str = "/sys/devices/system/cpu/cpufreq";

pub fn num_policies() -> Result<usize> {
    let policy_prefix = "policy".as_bytes();
    std::fs::read_dir(SYSFS_DIR)?.try_fold(0, |acc, res| match (acc, res) {
        (acc, Ok(inode))
            if {
                let name = inode.file_name();
                let name = name.as_bytes();
                // Not sure if this is robust enough.
                // This will make sure that the name starts with "policy" but does not equal
                // exactly "policy". It does not, however, check for non-numeric
                // characters at the end of the "policy" prefix. Realistically,
                // we need to pattern match here, something like `^policy[0-9]+$`.
                // Unfortunately regex is heavier than I want it to be, so that's not a good
                // option.
                name.len() >= policy_prefix.len() && &name[..policy_prefix.len()] == policy_prefix
            } =>
        {
            Ok(acc + 1)
        }
        (acc, Ok(_)) => Ok(acc),
        (_, Err(e)) => Err(e.into()),
    })
}

impl_sysfs_attrs! {
    /// List of online CPUs belonging to this policy (i.e. sharing the
    /// hardware performance scaling interface represented by the policyX
    /// policy object).
    pub sysfs_attr affected_cpus(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/affected_cpus`
        /// *sysfs* attribute.**
        read: |text| text.split(' ').map(|int| int.parse().unwrap()).collect() => Vec<usize>,
    }

    /// If the platform firmware (BIOS) tells the OS to apply an upper limit
    /// to CPU frequencies, that limit will be reported through this
    /// attribute (if present).
    ///
    /// The existence of the limit may be a result of some (often
    /// unintentional) BIOS settings, restrictions coming from a service
    /// processor or another BIOS/HW-based mechanisms.
    ///
    /// This does not cover ACPI thermal limitations which can be discovered
    /// through a generic thermal driver.
    ///
    /// This attribute is not present if the scaling driver in use does not
    /// support it.
    pub sysfs_attr bios_limit(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/bios_limit`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
    }

    /// Current frequency of the CPUs belonging to this policy as obtained
    /// from the hardware (in KHz).
    ///
    /// This is expected to be the frequency the hardware actually runs at.
    /// If that frequency cannot be determined, this attribute should not be
    /// present.
    pub sysfs_attr cpuinfo_cur_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/cpuinfo_cur_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
    }

    /// Maximum possible operating frequency the CPUs belonging to this
    /// policy can run at (in kHz).
    pub sysfs_attr cpuinfo_max_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/cpuinfo_max_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
    }

    /// Minimum possible operating frequency the CPUs belonging to this
    /// policy can run at (in kHz).
    pub sysfs_attr cpuinfo_min_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/cpuinfo_min_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
    }

    /// The time it takes to switch the CPUs belonging to this policy from
    /// one P-state to another, in nanoseconds.
    ///
    /// If unknown or if known to be so high that the scaling driver does
    /// not work with the ondemand governor, -1 (CPUFREQ_ETERNAL) will be
    /// returned by reads from this attribute.
    pub sysfs_attr cpuinfo_transition_latency(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/cpuinfo_transition_latency`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => isize,
    }

    /// List of all (online and offline) CPUs belonging to this policy.
    pub sysfs_attr related_cpus(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/related_cpus`
        /// *sysfs* attribute.**
        read: |text| text.split(' ').map(|int| int.parse().unwrap()).collect() => Vec<usize>,
    }

    /// List of CPUFreq scaling governors present in the kernel that can be
    /// attached to this policy or (if the intel_pstate scaling driver is in
    /// use) list of scaling algorithms provided by the driver that can be
    /// applied to this policy.
    ///
    /// [Note that some governors are modular and it may be necessary to
    /// load a kernel module for the governor held by it to become available
    /// and be listed by this attribute.]
    pub sysfs_attr scaling_available_governors(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_available_governors`
        /// *sysfs* attribute.**
        read: |text| text.split(' ').map(ToOwned::to_owned).collect() => Vec<String>,
    }

    /// Current frequency of all of the CPUs belonging to this policy
    /// (in kHz).
    ///
    /// In the majority of cases, this is the frequency of the last P-state
    /// requested by the scaling driver from the hardware using the scaling
    /// interface provided by it, which may or may not reflect the frequency
    /// the CPU is actually running at (due to hardware design and other
    /// limitations).
    ///
    /// Some architectures (e.g. x86) may attempt to provide information
    /// more precisely reflecting the current CPU frequency through this
    /// attribute, but that still may not be the exact current CPU frequency
    /// as seen by the hardware at the moment.
    pub sysfs_attr scaling_cur_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_cur_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
    }

    /// The scaling driver currently in use.
    pub sysfs_attr scaling_driver(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_driver`
        /// *sysfs* attribute.**
        read: ToOwned::to_owned => String,
    }

    /// The scaling governor currently attached to this policy or (if the
    /// intel_pstate scaling driver is in use) the scaling algorithm
    /// provided by the driver that is currently applied to this policy.
    ///
    /// This attribute is read-write and writing to it will cause a new
    /// scaling governor to be attached to this policy or a new scaling
    /// algorithm provided by the scaling driver to be applied to it (in the
    /// intel_pstate case), as indicated by the string written to this
    /// attribute (which must be one of the names listed by the
    /// scaling_available_governors attribute described above).
    pub sysfs_attr scaling_governor(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_governor`
        /// *sysfs* attribute.**
        read: ToOwned::to_owned => String,
        /// **This is the setter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_governor`
        /// *sysfs* attribute.**
        write: |gov: &str| -> format_args!("{}", gov),
    }

    /// Maximum frequency the CPUs belonging to this policy are allowed to
    /// be running at (in kHz).
    ///
    /// This attribute is read-write and writing a string representing an
    /// integer to it will cause a new limit to be set (it must not be lower
    /// than the value of the scaling_min_freq attribute).
    pub sysfs_attr scaling_max_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_max_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
        /// **This is the setter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_max_freq`
        /// *sysfs* attribute.**
        write: |freq: usize| -> format_args!("{}", freq),
    }

    /// Minimum frequency the CPUs belonging to this policy are allowed to
    /// be running at (in kHz).
    ///
    /// This attribute is read-write and writing a string representing a
    /// non-negative integer to it will cause a new limit to be set (it must
    /// not be higher than the value of the scaling_max_freq attribute).
    pub sysfs_attr scaling_min_freq(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_min_freq`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
        /// **This is the setter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_min_freq`
        /// *sysfs* attribute.**
        write: |freq: usize| -> format_args!("{}", freq),
    }

    /// This attribute is functional only if the userspace scaling governor
    /// is attached to the given policy.
    ///
    /// It returns the last frequency requested by the governor (in kHz) or
    /// can be written to in order to set a new frequency for the policy.
    pub sysfs_attr scaling_setspeed(cpu: usize) in "{SYSFS_DIR}/policy{cpu}" {
        /// **This is the getter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_setspeed`
        /// *sysfs* attribute.**
        read: |text| text.parse().unwrap() => usize,
        /// **This is the setter function for the
        /// `/sys/devices/system/cpu/cpufreq/policy*/scaling_setspeed`
        /// *sysfs* attribute.**
        write: |freq: usize| -> format_args!("{}", freq),
    }
}
