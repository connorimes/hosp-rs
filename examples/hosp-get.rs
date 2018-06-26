extern crate hosp;
extern crate hidapi;

use hidapi::HidApi;
use hosp::*;

const DELAY_MS: u64 = 1;
const READ_RETRIES: u32 = 250;
const MAX_FAILURES: u32 = 3;

fn main() {
    // first we need a hidapi handle
    let hid = HidApi::new().expect("Failed to initialize HID");
    // now open the ODROID Smart Power (`hosp` lifetime <= `hid` lifetime)
    let hosp = HospDevice::from_hid(&hid).expect("Failed to open ODROID Smart Power connection");
    // now get the info to print
    let version = util::retry_get(|| hosp.get_version(), READ_RETRIES, DELAY_MS, MAX_FAILURES);
    let status = util::retry_get(|| hosp.get_status(), READ_RETRIES, DELAY_MS, MAX_FAILURES);
    let data = util::retry_get(|| hosp.get_data(), READ_RETRIES, DELAY_MS, MAX_FAILURES);
    // print results
    println!("Version: {}", version);
    println!("On: {}", status.on);
    println!("Started: {}", status.started);
    println!("Millivolts: {}", data.m_volts);
    println!("Milliamps: {}", data.m_amps.unwrap_or(0));
    println!("Milliwatts: {}", data.m_watts.unwrap_or(0));
    println!("Milliwatt-hours: {}", data.m_watt_hours.unwrap_or(0));
}
