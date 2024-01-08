#[path = "shared/common.rs"]
mod common;

use common::print_object;
use sysfs::api::cpu::count_cpus;

fn main() {
    for cpu in 0..count_cpus().unwrap() {
        print_object!(
            in sysfs::api::cpu::acpi_cppc
            ["/sys/devices/system/cpu/cpu{}/acpi_cppc", cpu] {
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
