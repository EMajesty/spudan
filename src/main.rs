use anyhow::{self};
// use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals},
    // nvs::EspDefaultNvsPartition,
    // wifi::EspWifi,
};

mod hub75;

// mod secrets;
// use secrets::{WIFI_PASSWORD, WIFI_SSID};

fn main() -> anyhow::Result<()>{
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    // let sysloop = EspSystemEventLoop::take()?;
    // let nvs = EspDefaultNvsPartition::take()?;
    // let mut delay = FreeRtos;

    let mut r1 = PinDriver::output(peripherals.pins.gpio25)?;
    let g1 = PinDriver::output(peripherals.pins.gpio26)?;
    let b1 = PinDriver::output(peripherals.pins.gpio27)?;
    let r2 = PinDriver::output(peripherals.pins.gpio14)?;
    let g2 = PinDriver::output(peripherals.pins.gpio12)?;
    let b2 = PinDriver::output(peripherals.pins.gpio13)?;
    let a = PinDriver::output(peripherals.pins.gpio23)?;
    let b = PinDriver::output(peripherals.pins.gpio19)?;
    let c = PinDriver::output(peripherals.pins.gpio5)?;
    let d = PinDriver::output(peripherals.pins.gpio17)?;
    let e = PinDriver::output(peripherals.pins.gpio32)?;
    let mut clk = PinDriver::output(peripherals.pins.gpio16)?;
    let lat = PinDriver::output(peripherals.pins.gpio4)?;
    let oe = PinDriver::output(peripherals.pins.gpio15)?;

    // let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;
    //
    // let config = Configuration::Client(ClientConfiguration {
    //     ssid: WIFI_SSID.try_into()?,
    //     password: WIFI_PASSWORD.try_into()?,
    //     auth_method: AuthMethod::WPA2Personal,
    //     ..Default::default()
    // });
    //
    // wifi.set_configuration(&config)?;
    // wifi.start()?;
    // wifi.connect()?;
    //
    // while !wifi.is_connected()? {
    //     log::info!("Waiting for connection");
    // }
    //
    // log::info!("Connected");
    
    r1.set_high()?;
    clk.set_high()?;
    clk.set_low()?;

    loop {
    }
}

// r1 2
// g1 4
// b1 5
// r2 18
// g2 19
// b2 21
// a 12
// b 13
// c 14
// d 15
// e 16
// clk 25
// lat 26
// oe 27
//
// r1 | g1
// b1 | gnd
// r2 | g2
// b2 | e
// a  | b
// c  | d
// clk| lat
// oe | gnd
