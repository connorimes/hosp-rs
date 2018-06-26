extern crate hosp;
extern crate hidapi;

use hidapi::HidApi;
use hosp::*;
use std::thread::sleep;
use std::time::Duration;

const DELAY_MS: u64 = 1;
const READ_RETRIES: u32 = 250;
const MAX_FAILURES: u32 = 3;

fn main() {
    // first we need a hidapi handle
    let hid = HidApi::new().expect("Failed to initialize HID");
    // now open the ODROID Smart Power (`hosp` lifetime <= `hid` lifetime)
    let hosp = HospDevice::from_hid(&hid).expect("Failed to open ODROID Smart Power connection");
    println!("Millivolts,Milliamps,Milliwatts,Milliwatt-hours");
    let ms = Duration::from_millis(util::REFRESH_INTERVAL_MS);
    loop {
        let data = util::retry_get(|| hosp.get_data(), READ_RETRIES, DELAY_MS, MAX_FAILURES);
        println!("{},{},{},{}", data.m_volts,
                                data.m_amps.unwrap_or(0),
                                data.m_watts.unwrap_or(0),
                                data.m_watt_hours.unwrap_or(0));
        sleep(ms);
    }
}
