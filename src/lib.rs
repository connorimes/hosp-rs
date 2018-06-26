//! This crate provides an API for managing an ODROID Smart Power over USB.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/hosp) and can be
//! used by adding `hosp` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! hosp = "0.1"
//! ```
//!
//! Getter functions return `Result` errors when I/O fails.
//! Assuming no I/O errors, an `Option` type is returned from getter functions.
//! `None` is a normal return value when the device replies to a read request
//! without providing any real data.
//! Users implement their own retry policy waiting for a `Some` return value.
//! ODROID Smart Power devices normally refresh at 10 Hz.

extern crate hidapi;

pub mod util;

pub use hidapi::HidApi;
use hidapi::HidDevice;

const VENDOR_ID: u16 = 0x04d8;
const PRODUCT_ID: u16 = 0x003f;

const REQUEST_DATA: u8 = 0x37;
const REQUEST_STARTSTOP: u8 = 0x80;
const REQUEST_STATUS: u8 = 0x81;
const REQUEST_ONOFF: u8 = 0x82;
const REQUEST_VERSION: u8 = 0x83;

const STATUS_ON: u8 = 0x01;
const STATUS_STARTED: u8 = 0x01;

const BUF_SIZE: usize = 65;

pub type HospResult<T> = Result<T, &'static str>;

/// Context object for managing the HID device.
pub struct HospDevice<'a> {
    dev: HidDevice<'a>,
    /// The read timeout in milliseconds: 0 by default, -1 for blocking reads
    pub timeout_ms: i32,
}

/// ODROID Smart Power device status info.
pub struct HospStatus {
    pub on: bool,
    pub started: bool,
}

/// ODROID Smart Power device data.
/// Optional fields are not available when the device is OFF.
pub struct HospData {
    pub m_volts: u32,
    pub m_amps: Option<u32>,
    pub m_watts: Option<u32>,
    pub m_watt_hours: Option<u32>,
}

/// Values are only accurate to 3 decimal places, so we don't lose any precision
fn to_milliunits(bytes: &[u8]) -> Option<u32> {
    match bytes[0] as char {
        '-' => None, // device is off
        _ => String::from_utf8_lossy(bytes).trim_left()
                                           .parse::<f32>()
                                           .ok()
                                           .map(|val| (val * 1000.0) as u32)
    }
}

impl<'a> HospDevice<'a> {
    /// Open the ODROID Smart Power
    pub fn from_hid(hid: &'a HidApi) -> HospResult<Self> {
        hid.open(VENDOR_ID, PRODUCT_ID).map(|dev| HospDevice{ dev: dev, timeout_ms: 0 })
    }

    fn write(&self, t: u8) -> HospResult<()> {
        self.dev.write(&[0, t]).map(|_| ())
    }

    fn read(&self, t: u8) -> HospResult<Option<[u8; BUF_SIZE]>> {
        let mut buf = [0; BUF_SIZE];
        buf[1] = t;
        // return Some only if response was actually set by device
        self.dev.read_timeout(&mut buf, self.timeout_ms).map(|_| if buf[0] == t { Some(buf) } else { None })
    }

    /// Try to get the version string
    pub fn get_version(&self) -> HospResult<Option<String>> {
        self.write(REQUEST_VERSION).and(self.read(REQUEST_VERSION).map(|opt| opt.map(|buf|
            String::from_utf8_lossy(&buf[1..17]).into_owned()
        )))
    }

    /// Try to get the device status
    pub fn get_status(&self) -> HospResult<Option<HospStatus>> {
        self.write(REQUEST_STATUS).and(self.read(REQUEST_STATUS).map(|opt| opt.map(|buf|
            HospStatus {
                on: buf[2] == STATUS_ON,
                started: buf[1] == STATUS_STARTED,
            }
        )))
    }

    /// Toggle the ON/OFF status
    pub fn toggle_onoff(&self) -> HospResult<()> {
        self.write(REQUEST_ONOFF)
    }

    /// Toggle the START/STOP status
    pub fn toggle_startstop(&self) -> HospResult<()> {
        self.write(REQUEST_STARTSTOP)
    }

    /// Try to get data from the device
    pub fn get_data(&self) -> HospResult<Option<HospData>> {
        // Reply when device is off: "7 5.000V  -.--- A -.---W  -.---Wh" followed by garbage characters
        // Dashes are replaced with actual values when device is on
        self.write(REQUEST_DATA).and(self.read(REQUEST_DATA).map(|opt| opt.map(|buf|
            HospData {
                // Volts are always shown, even when device is off
                m_volts: to_milliunits(&buf[2..7]).unwrap(),
                m_amps: to_milliunits(&buf[10..15]),
                m_watts: to_milliunits(&buf[17..23]),
                m_watt_hours: to_milliunits(&buf[24..31])
            }
        )))
    }
}
