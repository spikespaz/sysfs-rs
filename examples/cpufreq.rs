#[path = "shared/common.rs"]
mod common;

use common::print_object;
use sysfs::api::cpu::num_cpus;

fn main() {
    for cpu in 0..num_cpus().unwrap() {
        print_object!(
            in sysfs::api::cpu::cpufreq
            ["/sys/devices/system/cpu/cpufreq/policy{}", cpu] {
                affected_cpus,
                bios_limit,
                cpuinfo_cur_freq,
                cpuinfo_max_freq,
                cpuinfo_min_freq,
                cpuinfo_transition_latency,
                related_cpus,
                scaling_available_governors,
                scaling_cur_freq,
                scaling_driver,
                scaling_governor,
                scaling_max_freq,
                scaling_min_freq,
                scaling_setspeed,
            }
        );
        println!();
    }
}
