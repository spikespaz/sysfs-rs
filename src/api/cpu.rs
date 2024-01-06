//! <https://www.kernel.org/doc/html/latest/admin-guide/pm/cpufreq.html?highlight=schedutil#policy-interface-in-sysfs>
use crate::lib::{sysfs_attrs, Result};

pub fn num_cpus() -> Result<usize> {
    std::fs::read_dir("/sys/devices/system/cpu/cpufreq")?.try_fold(0, |acc, res| match (acc, res) {
        (acc, Ok(inode))
            if {
                let name = inode.file_name();
                let name = name.to_string_lossy();
                name.starts_with("policy")
                    && name["policy".len()..].chars().all(|ch| ch.is_ascii_digit())
            } =>
        {
            Ok(acc + 1)
        }
        (acc, Ok(_)) => Ok(acc),
        (_, Err(e)) => Err(e.into()),
    })
}

/// <https://www.kernel.org/doc/html/latest/admin-guide/pm/cpufreq.html#policy-interface-in-sysfs>
#[sysfs_attrs(in "/sys/devices/system/cpu/cpufreq/policy{cpu}")]
pub mod cpufreq {
    use crate::lib::sysfs;

    /// List of online CPUs belonging to this policy (i.e. sharing the
    /// hardware performance scaling interface represented by the policyX
    /// policy object).
    #[sysfs]
    pub fn affected_cpus(cpu: usize) -> Vec<usize> {
        let read = |text: &str| text.split(' ').map(|int| int.parse().unwrap()).collect();
        ..
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
    #[sysfs]
    pub fn bios_limit(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Current frequency of the CPUs belonging to this policy as obtained
    /// from the hardware (in KHz).
    ///
    /// This is expected to be the frequency the hardware actually runs at.
    /// If that frequency cannot be determined, this attribute should not be
    /// present.
    #[sysfs]
    pub fn cpuinfo_cur_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Maximum possible operating frequency the CPUs belonging to this
    /// policy can run at (in kHz).
    #[sysfs]
    pub fn cpuinfo_max_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Minimum possible operating frequency the CPUs belonging to this
    /// policy can run at (in kHz).
    #[sysfs]
    pub fn cpuinfo_min_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// The time it takes to switch the CPUs belonging to this policy from
    /// one P-state to another, in nanoseconds.
    ///
    /// If unknown or if known to be so high that the scaling driver does
    /// not work with the ondemand governor, -1 (CPUFREQ_ETERNAL) will be
    /// returned by reads from this attribute.
    #[sysfs]
    pub fn cpuinfo_transition_latency(cpu: usize) -> isize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// List of all (online and offline) CPUs belonging to this policy.
    #[sysfs]
    pub fn related_cpus(cpu: usize) -> Vec<usize> {
        let read = |text: &str| text.split(' ').map(|int| int.parse().unwrap()).collect();
        ..
    }

    /// List of CPUFreq scaling governors present in the kernel that can be
    /// attached to this policy or (if the intel_pstate scaling driver is in
    /// use) list of scaling algorithms provided by the driver that can be
    /// applied to this policy.
    ///
    /// [Note that some governors are modular and it may be necessary to
    /// load a kernel module for the governor held by it to become available
    /// and be listed by this attribute.]
    #[sysfs]
    pub fn scaling_available_governors(cpu: usize) -> Vec<String> {
        let read = |text: &str| text.split(' ').map(str::to_owned).collect();
        ..
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
    #[sysfs]
    pub fn scaling_cur_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// The scaling driver currently in use.
    #[sysfs]
    pub fn scaling_driver(cpu: usize) -> String {
        let read = str::to_owned;
        ..
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
    #[sysfs]
    pub fn scaling_governor(cpu: usize) -> String {
        let read = str::to_owned;
        let write = |gov: &str| gov.to_owned();
        ..
    }

    /// Maximum frequency the CPUs belonging to this policy are allowed to
    /// be running at (in kHz).
    ///
    /// This attribute is read-write and writing a string representing an
    /// integer to it will cause a new limit to be set (it must not be lower
    /// than the value of the scaling_min_freq attribute).
    #[sysfs]
    pub fn scaling_max_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |khz: usize| khz.to_string();
        ..
    }

    /// Minimum frequency the CPUs belonging to this policy are allowed to
    /// be running at (in kHz).
    ///
    /// This attribute is read-write and writing a string representing a
    /// non-negative integer to it will cause a new limit to be set (it must
    /// not be higher than the value of the scaling_max_freq attribute).
    #[sysfs]
    pub fn scaling_min_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |khz: usize| khz.to_string();
        ..
    }

    /// This attribute is functional only if the userspace scaling governor
    /// is attached to the given policy.
    ///
    /// It returns the last frequency requested by the governor (in kHz) or
    /// can be written to in order to set a new frequency for the policy.
    #[sysfs]
    pub fn scaling_setspeed(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |khz: usize| khz.to_string();
        ..
    }
}

// Currently the functions in here are all prefixed with `amd_pstate`.
// The attribute files themselves are all in the `cpufreq` subdirectory.
//
// I thought it was best to put them in a separate module,
// because it is a separate feature that has to be enabled by the kernel
// parameter `amd_pstate=` as either `active`, `passive`, or `guided`.
//
// Because they are now in a separate module, I think it is best to remove the
// prefix.
#[sysfs_attrs(in "/sys/devices/system/cpu/cpufreq/policy{cpu}")]
pub mod amd_pstate {
    use crate::lib::sysfs;

    /// Maximum CPPC performance and CPU frequency that the driver is allowed to
    /// set, in percent of the maximum supported CPPC performance level (the
    /// highest performance supported in AMD CPPC Performance Capability).
    /// In some ASICs, the highest CPPC performance is not the one in the _CPC
    /// table, so we need to expose it to sysfs. If boost is not active, but
    /// still supported, this maximum frequency will be larger than the one in
    /// cpuinfo. This attribute is read-only.
    #[sysfs]
    pub fn amd_pstate_highest_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// See documentation for [`amd_pstate_highest_perf`].
    #[sysfs]
    pub fn amd_pstate_max_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// The lowest non-linear CPPC CPU frequency that the driver is allowed to
    /// set, in percent of the maximum supported CPPC performance level.
    /// (Please see the lowest non-linear performance in AMD CPPC Performance
    /// Capability.) This attribute is read-only.
    #[sysfs]
    pub fn amd_pstate_lowest_nonlinear_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// A list of all the supported EPP preferences that could be used for
    /// energy_performance_preference on this system. These profiles represent
    /// different hints that are provided to the low-level firmware about the
    /// user's desired energy vs efficiency tradeoff. default represents the epp
    /// value is set by platform firmware. This attribute is read-only.
    #[sysfs]
    pub fn energy_performance_available_preferences(cpu: usize) -> Vec<String> {
        let read = |text: &str| text.split(' ').map(str::to_owned).collect();
        ..
    }

    /// The current energy performance preference can be read from this
    /// attribute. and user can change current preference according to energy or
    /// performance needs Please get all support profiles list from
    /// energy_performance_available_preferences attribute, all the profiles are
    /// integer values defined between 0 to 255 when EPP feature is enabled by
    /// platform firmware, if EPP feature is disabled, driver will ignore the
    /// written value This attribute is read-write.
    #[sysfs]
    pub fn energy_performance_preference(cpu: usize) -> String {
        let read = str::to_owned;
        let write = |epp: &str| epp.to_owned();
        ..
    }
}

/// <https://www.kernel.org/doc/html/latest/admin-guide/acpi/cppc_sysfs.html>
#[sysfs_attrs(in "/sys/devices/system/cpu/cpu{cpu}")]
pub mod acpi_cppc {
    use crate::lib::sysfs;

    /// Highest performance of this processor (abstract scale).
    #[sysfs]
    pub fn highest_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Highest sustained performance of this processor (abstract scale).
    #[sysfs]
    pub fn nominal_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Lowest performance of this processor with nonlinear power savings (abstract scale).
    #[sysfs]
    pub fn lowest_nonlinear_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Lowest performance of this processor (abstract scale).
    #[sysfs]
    pub fn lowest_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// CPU frequency corresponding to lowest_perf (in MHz).
    #[sysfs]
    pub fn lowest_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// CPU frequency corresponding to nominal_perf (in MHz). The above
    /// frequencies should only be used to report processor performance in
    /// frequency instead of abstract scale. These values should not be used
    /// for any functional decisions.
    #[sysfs]
    pub fn nominal_freq(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Includes both Reference and delivered performance counter. Reference
    /// counter ticks up proportional to processor's reference performance.
    /// Delivered counter ticks up proportional to processor's delivered
    /// performance.
    #[sysfs]
    pub fn feedback_ctrs(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Minimum time for the feedback counters to wraparound (seconds).
    #[sysfs]
    pub fn wraparound_time(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Performance level at which reference performance counter accumulates
    /// (abstract scale).
    #[sysfs]
    pub fn reference_perf(cpu: usize) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }
}
