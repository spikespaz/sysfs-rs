#[path = "shared/common.rs"]
mod common;

use common::print_object;
use sysfs::api::cpu::count_cpus;

fn main() {
    for cpu in 0..count_cpus().unwrap() {
        print_object!(
            in sysfs::api::cpu::amd_pstate
            ["/sys/devices/system/cpu/cpufreq/policy{}", cpu] {
                amd_pstate_highest_perf,
                amd_pstate_max_freq,
                amd_pstate_lowest_nonlinear_freq,
                energy_performance_available_preferences,
                energy_performance_preference,
            }
        );
        println!();
    }
}
