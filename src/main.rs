use aht20::Aht20;
use esp_idf_hal::{
    delay,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use esp_idf_sys as _;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let config = I2cConfig::new().baudrate(115200.Hz());
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();

    let mut aht20 = Aht20::new(i2c, delay::FreeRtos).unwrap();
    loop {
        let (h, t) = aht20.read().unwrap();
        println!("Temperature: {:.2}C", t.celsius());
        println!("Humidity: {:.2}%", h.rh());
        delay::FreeRtos::delay_ms(1000);
    }
}
