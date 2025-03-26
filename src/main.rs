use anyhow::{self};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals},
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};
use hub75::Hub75;

mod secrets;
use secrets::{WIFI_PASSWORD, WIFI_SSID};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut delay = FreeRtos;

    let r1 = PinDriver::output(peripherals.pins.gpio25).unwrap();
    let g1 = PinDriver::output(peripherals.pins.gpio26).unwrap();
    let b1 = PinDriver::output(peripherals.pins.gpio27).unwrap();
    let r2 = PinDriver::output(peripherals.pins.gpio14).unwrap();
    let g2 = PinDriver::output(peripherals.pins.gpio12).unwrap();
    let b2 = PinDriver::output(peripherals.pins.gpio13).unwrap();
    let a = PinDriver::output(peripherals.pins.gpio23).unwrap();
    let b = PinDriver::output(peripherals.pins.gpio19).unwrap();
    let c = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let d = PinDriver::output(peripherals.pins.gpio17).unwrap();
    let e = PinDriver::output(peripherals.pins.gpio32).unwrap();
    let clk = PinDriver::output(peripherals.pins.gpio16).unwrap();
    let lat = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let oe = PinDriver::output(peripherals.pins.gpio15).unwrap();

    let display_pins = (r1, g1, b1, r2, g2, b2, a, b, c, d, e, clk, lat, oe);

    let mut display = Hub75::new(display_pins, 4);

    let mut wifi = EspWifi::new(peripherals.modem, sysloop, Some(nvs))?;

    let config = Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASSWORD.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    });

    wifi.set_configuration(&config)?;
    wifi.start()?;
    wifi.connect()?;

    while !wifi.is_connected()? {
        log::info!("Waiting for connection");
    }

    log::info!("Connected");

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    loop {
        // display.output(&mut delay)?;
        led.set_high()?;
        FreeRtos::delay_ms(500);
        led.set_low()?;
        FreeRtos::delay_ms(500);
    }
}

fn clock_tick() {
}
