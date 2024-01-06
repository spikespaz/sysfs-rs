use crate::lib::sysfs_attrs;

/// <https://www.kernel.org/doc/html/latest/power/power_supply_class.html>
/// <https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-power>
#[sysfs_attrs(in "/sys/class/power_supply/{psu}")]
pub mod power_supply {
    use crate::lib::sysfs;

    /// Reports the name of the device manufacturer.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented as string
    #[sysfs]
    pub fn manufacturer(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Reports the name of the device model.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented as string
    #[sysfs]
    pub fn model_name(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Reports the serial number of the device.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented as string
    #[sysfs]
    pub fn serial_number(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Describes the main type of the supply.
    ///
    /// Access: Read
    ///
    /// Valid values: "Battery", "UPS", "Mains", "USB", "Wireless"
    #[sysfs]
    pub fn r#type(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Battery:
    ///
    /// Reports an average IBAT current reading for the battery, over
    /// a fixed period. Normally devices will provide a fixed interval
    /// in which they average readings to smooth out the reported
    /// value.
    ///
    /// USB:
    ///
    /// Reports an average IBUS current reading over a fixed period.
    /// Normally devices will provide a fixed interval in which they
    /// average readings to smooth out the reported value.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microamps. Negative values are
    /// used for discharging batteries, positive values for charging
    /// batteries and for USB IBUS current.
    #[sysfs]
    pub fn current_avg(psu: &str) -> isize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports the maximum IBAT current allowed into the battery.
    ///
    /// USB:
    ///
    /// Reports the maximum IBUS current the supply can support.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn current_max(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports an instant, single IBAT current reading for the
    /// battery. This value is not averaged/smoothed.
    ///
    /// Access: Read
    ///
    /// USB:
    ///
    /// Reports the IBUS current supplied now. This value is generally
    /// read-only reporting, unless the 'online' state of the supply
    /// is set to be programmable, in which case this value can be set
    /// within the reported min/max range.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microamps. Negative values are
    /// used for discharging batteries, positive values for charging
    /// batteries and for USB IBUS current.
    #[sysfs]
    pub fn current_now(psu: &str) -> isize {
        let read = |text: &str| text.parse().unwrap();
        let write = |max: isize| max.to_string();
        ..
    }

    /// Battery:
    ///
    /// Reports the current TBAT battery temperature reading.
    ///
    /// USB:
    ///
    /// Reports the current supply temperature reading. This would
    /// normally be the internal temperature of the device itself
    /// (e.g TJUNC temperature of an IC)
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in 1/10 Degrees Celsius
    #[sysfs]
    pub fn temp(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Maximum TBAT temperature trip-wire value where the supply will
    /// notify user-space of the event.
    ///
    /// USB:
    ///
    /// Maximum supply temperature trip-wire value where the supply
    /// will notify user-space of the event.
    ///
    /// This is normally used for the charging scenario where
    /// user-space needs to know if the temperature has crossed an
    /// upper threshold so it can take appropriate action (e.g. warning
    /// user that the temperature is critically high, and charging has
    /// stopped).
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in 1/10 Degrees Celsius
    #[sysfs]
    pub fn temp_alert_max(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Minimum TBAT temperature trip-wire value where the supply will
    /// notify user-space of the event.
    ///
    /// USB:
    ///
    /// Minimum supply temperature trip-wire value where the supply
    /// will notify user-space of the event.
    ///
    /// This is normally used for the charging scenario where user-space
    /// needs to know if the temperature has crossed a lower threshold
    /// so it can take appropriate action (e.g. warning user that
    /// temperature level is high, and charging current has been
    /// reduced accordingly to remedy the situation).
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in 1/10 Degrees Celsius
    #[sysfs]
    pub fn temp_alert_min(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports the maximum allowed TBAT battery temperature for
    /// charging.
    ///
    /// USB:
    ///
    /// Reports the maximum allowed supply temperature for operation.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in 1/10 Degrees Celsius
    #[sysfs]
    pub fn temp_max(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports the minimum allowed TBAT battery temperature for
    /// charging.
    ///
    /// USB:
    ///
    /// Reports the minimum allowed supply temperature for operation.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in 1/10 Degrees Celsius
    #[sysfs]
    pub fn temp_min(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports the maximum safe VBAT voltage permitted for the
    /// battery, during charging.
    ///
    /// USB:
    ///
    /// Reports the maximum VBUS voltage the supply can support.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microvolts
    #[sysfs]
    pub fn voltage_max(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:

    /// Reports the minimum safe VBAT voltage permitted for the
    /// battery, during discharging.

    /// USB:
    ///
    /// Reports the minimum VBUS voltage the supply can support.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microvolts
    #[sysfs]
    pub fn voltage_min(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Battery:
    ///
    /// Reports an instant, single VBAT voltage reading for the
    /// battery. This value is not averaged/smoothed.
    ///
    /// Access: Read
    ///
    /// USB:
    ///
    /// Reports the VBUS voltage supplied now. This value is generally
    /// read-only reporting, unless the 'online' state of the supply
    /// is set to be programmable, in which case this value can be set
    /// within the reported min/max range.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microvolts
    #[sysfs]
    pub fn voltage_now(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |uvolts: usize| uvolts.to_string();
        ..
    }

    /// Fine grain representation of battery capacity.
    ///
    /// Access: Read
    ///
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn capacity(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        let write = |percent: f32| ((percent * 100.0).round() as u8).to_string();
        ..
    }

    /// Maximum battery capacity trip-wire value where the supply will
    /// notify user-space of the event. This is normally used for the
    /// battery discharging scenario where user-space needs to know the
    /// battery has dropped to an upper level so it can take
    /// appropriate action (e.g. warning user that battery level is
    /// low).
    ///
    /// Access: Read, Write
    ///
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn capacity_alert_max(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        let write = |percent: f32| ((percent * 100.0).round() as u8).to_string();
        ..
    }

    /// Minimum battery capacity trip-wire value where the supply will
    /// notify user-space of the event. This is normally used for the
    /// battery discharging scenario where user-space needs to know the
    /// battery has dropped to a lower level so it can take
    /// appropriate action (e.g. warning user that battery level is
    /// critically low).
    ///
    /// Access: Read, Write
    ///
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn capacity_alert_min(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        let write = |percent: f32| ((percent * 100.0).round() as u8).to_string();
        ..
    }

    /// Battery capacity measurement becomes unreliable without
    /// recalibration. This values provides the maximum error
    /// margin expected to exist by the fuel gauge in percent.
    /// Values close to 0% will be returned after (re-)calibration
    /// has happened. Over time the error margin will increase.
    /// 100% means, that the capacity related values are basically
    /// completely useless.
    ///
    /// Access: Read
    ///
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn capacity_error_margin(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        ..
    }

    /// Coarse representation of battery capacity.
    ///
    /// Access: Read
    ///
    /// Valid values: "Unknown", "Critical", "Low", "Normal", "High", "Full"
    #[sysfs]
    pub fn capacity_level(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Maximum allowable charging current. Used for charge rate
    /// throttling for thermal cooling or improving battery health.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn charge_control_limit(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |uamps: usize| uamps.to_string();
        ..
    }

    /// Maximum legal value for the charge_control_limit property.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn charge_control_limit_max(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Represents a battery percentage level, below which charging will
    /// begin.
    ///
    /// Access: Read, Write
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn charge_control_start_threshold(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        let write = |percent: f32| ((percent * 100.0).round() as u8).to_string();
        ..
    }

    /// Represents a battery percentage level, above which charging will
    /// stop. Not all hardware is capable of setting this to an arbitrary
    /// percentage. Drivers will round written values to the nearest
    /// supported value. Reading back the value will show the actual
    /// threshold set by the driver.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: 0 - 100 (percent)
    #[sysfs]
    pub fn charge_control_end_threshold(psu: &str) -> f32 {
        let read = |text: &str| text.parse::<f32>().unwrap() / 100.0;
        let write = |percent: f32| ((percent * 100.0).round() as u8).to_string();
        ..
    }

    /// Represents the type of charging currently being applied to the
    /// battery. "Trickle", "Fast", and "Standard" all mean different
    /// charging speeds. "Adaptive" means that the charger uses some
    /// algorithm to adjust the charge rate dynamically, without
    /// any user configuration required. "Custom" means that the charger
    /// uses the charge_control_* properties as configuration for some
    /// different algorithm. "Long Life" means the charger reduces its
    /// charging rate in order to prolong the battery health. "Bypass"
    /// means the charger bypasses the charging path around the
    /// integrated converter allowing for a "smart" wall adaptor to
    /// perform the power conversion externally.
    ///
    /// Access: Read, Write
    ///
    /// Valid values:
    /// "Unknown", "N/A", "Trickle", "Fast", "Standard",
    /// "Adaptive", "Custom", "Long Life", "Bypass"
    #[sysfs]
    pub fn charge_type(psu: &str) -> String {
        let read = str::to_owned;
        let write = |charge_type: &str| charge_type.to_owned();
        ..
    }

    /// Reports the charging current value which is used to determine
    /// when the battery is considered full and charging should end.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn charge_term_current(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Reports the health of the battery or battery side of charger
    /// functionality.
    ///
    /// Access: Read
    ///
    /// Valid values:
    /// "Unknown", "Good", "Overheat", "Dead",
    /// "Over voltage", "Unspecified failure", "Cold",
    /// "Watchdog timer expire", "Safety timer expire",
    /// "Over current", "Calibration required", "Warm",
    /// "Cool", "Hot", "No battery"
    #[sysfs]
    pub fn health(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Reports the charging current applied during pre-charging phase
    /// for a battery charge cycle.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn precharge_current(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Reports whether a battery is present or not in the system. If the
    /// property does not exist, the battery is considered to be present.
    ///
    /// Access: Read
    ///
    /// Valid values: 0 (Absent), 1 (Present)
    #[sysfs]
    pub fn present(psu: &str) -> bool {
        let read = |text: &str| 1 == text.parse::<u8>().unwrap();
        ..
    }

    /// Represents the charging status of the battery. Normally this
    /// is read-only reporting although for some supplies this can be
    /// used to enable/disable charging to the battery.
    ///
    /// Access: Read, Write
    ///
    /// Valid values:
    /// "Unknown", "Charging", "Discharging", "Not charging", "Full"
    #[sysfs]
    pub fn status(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Represents the charging behaviour.
    ///
    /// Access: Read, Write
    ///
    /// Valid values:
    ///
    /// | Value             | Meaning                                  |
    /// |-------------------|------------------------------------------|
    /// | `auto`            | Charge normally, respect thresholds      |
    /// | `inhibit-charge`  | Do not charge while AC is attached       |
    /// | `force-discharge` | Force discharge while AC is attached     |
    #[sysfs]
    pub fn charge_behaviour(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Describes the battery technology supported by the supply.
    ///
    /// Access: Read
    ///
    /// Valid values:
    /// "Unknown", "NiMH", "Li-ion", "Li-poly", "LiFe", "NiCd", "LiMn"
    #[sysfs]
    pub fn technology(psu: &str) -> String {
        let read = str::to_owned;
        ..
    }

    /// Reports an average VBAT voltage reading for the battery, over a
    /// fixed period. Normally devices will provide a fixed interval in
    /// which they average readings to smooth out the reported value.
    ///
    /// Access: Read
    ///
    /// Valid values: Represented in microvolts
    #[sysfs]
    pub fn voltage_avg(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Reports the number of full charge + discharge cycles the
    /// battery has undergone.
    ///
    /// Access: Read
    ///
    /// Valid values:
    ///
    /// | Integer Value | Description                       |
    /// |---------------|-----------------------------------|
    /// | > 0           | representing full cycles          |
    /// | = 0           | cycle_count info is not available |
    #[sysfs]
    pub fn cycle_count(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        ..
    }

    /// Details the incoming IBUS current limit currently set in the
    /// supply. Normally this is configured based on the type of
    /// connection made (e.g. A configured SDP should output a maximum
    /// of 500mA so the input current limit is set to the same value).
    /// Use preferably input_power_limit, and for problems that can be
    /// solved using power limit use input_current_limit.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microamps
    #[sysfs]
    pub fn input_current_limit(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |uamps: usize| uamps.to_string();
        ..
    }
    /// This entry configures the incoming VBUS voltage limit currently
    /// set in the supply. Normally this is configured based on
    /// system-level knowledge or user input (e.g. This is part of the
    /// Pixel C's thermal management strategy to effectively limit the
    /// input power to 5V when the screen is on to meet Google's skin
    /// temperature targets). Note that this feature should not be
    /// used for safety critical things.
    /// Use preferably input_power_limit, and for problems that can be
    /// solved using power limit use input_voltage_limit.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microvolts
    #[sysfs]
    pub fn input_voltage_limit(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |uvolts: usize| uvolts.to_string();
        ..
    }

    /// This entry configures the incoming power limit currently set
    /// in the supply. Normally this is configured based on
    /// system-level knowledge or user input. Use preferably this
    /// feature to limit the incoming power and use current/voltage
    /// limit only for problems that can be solved using power limit.
    ///
    /// Access: Read, Write
    ///
    /// Valid values: Represented in microwatts
    #[sysfs]
    pub fn input_power_limit(psu: &str) -> usize {
        let read = |text: &str| text.parse().unwrap();
        let write = |uwatts: usize| uwatts.to_string();
        ..
    }

    /// Indicates if VBUS is present for the supply. When the supply is
    /// online, and the supply allows it, then it's possible to switch
    /// between online states (e.g. Fixed -> Programmable for a PD_PPS
    /// USB supply so voltage and current can be controlled).
    ///
    /// Access: Read, Write
    ///
    /// Valid values:
    ///
    /// | Value | State                  | Description                    |
    /// |-------|------------------------|--------------------------------|
    /// | 0     | Offline                |                                |
    /// | 1     | Online Fixed           | Fixed Voltage Supply           |
    /// | 2     | Online Programmable    | Programmable Voltage Supply    |
    #[sysfs]
    pub fn online(psu: &str) -> u8 {
        let read = |text: &str| text.parse().unwrap();
        let write = |state: u8| state.to_string();
        ..
    }

    /// Reports what type of USB connection is currently active for
    /// the supply, for example it can show if USB-PD capable source
    /// is attached.
    ///
    /// Access: Read-Only
    ///
    /// Valid values:
    /// "Unknown", "SDP", "DCP", "CDP", "ACA", "C", "PD",
    /// "PD_DRP", "PD_PPS", "BrickID"
    #[sysfs]
    pub fn usb_type(psu: &str) -> String {
        let read = |text: &str| text.parse().unwrap();
        ..
    }
}
