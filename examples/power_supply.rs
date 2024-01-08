#[path = "shared/common.rs"]
mod common;

use common::print_object;

fn main() {
    print_object! {
        in sysfs::api::psu::power_supply
        ["/sys/class/power_supply/{}", "BAT0"] {
            manufacturer,
            model_name,
            serial_number,
            r#type,
            current_avg,
            current_max,
            current_now,
            temp,
            temp_alert_max,
            temp_alert_min,
            temp_max,
            temp_min,
            voltage_max,
            voltage_min,
            voltage_now,
            capacity,
            capacity_alert_max,
            capacity_alert_min,
            capacity_error_margin,
            capacity_level,
            charge_control_limit,
            charge_control_limit_max,
            charge_control_start_threshold,
            charge_control_end_threshold,
            charge_type,
            charge_term_current,
            health,
            precharge_current,
            present,
            status,
            charge_behaviour,
            technology,
            voltage_avg,
            cycle_count,
            input_current_limit,
            input_voltage_limit,
            input_power_limit,
            online,
            usb_type,
        }
    }
}
