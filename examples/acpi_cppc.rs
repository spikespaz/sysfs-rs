#[path = "shared/common.rs"]
mod common;

use common::print_object;
use sysfs::api::cpu::num_cpus;

fn main() {
    for cpu in 0..num_cpus().unwrap() {
        print_object!(
            in sysfs::api::cpu::acpi_cppc
            ["/sys/devices/system/cpu/cpufreq/policy{}", cpu] {
                highest_perf,
                nominal_perf,
                lowest_nonlinear_perf,
                lowest_perf,
                lowest_freq,
                nominal_freq,
                feedback_ctrs,
                wraparound_time,
                reference_perf,
            }
        );
        println!();
    }
}
