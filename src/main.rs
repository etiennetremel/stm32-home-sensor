#![no_std]
#![no_main]

use panic_semihosting as _;

use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};
use scd30::scd30::Scd30;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

extern crate shared_bus;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let bus = shared_bus::BusManagerCortexM::new(i2c);

    let mut delay = cp.SYST.delay(&clocks);

    const LCD_ADDRESS: u8 = 0x27;
    const SCD30_ADDRESS: u8 = 0x61;

    // init display
    let mut lcd =
        HD44780::new_i2c(bus.acquire_i2c(), LCD_ADDRESS, &mut delay).expect("Init LCD failed");
    lcd.reset(&mut delay).unwrap();
    lcd.clear(&mut delay).unwrap();
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        },
        &mut delay,
    )
    .unwrap();

    lcd.write_str("Getting ready" as &str, &mut delay).unwrap();

    // init scd30 sensor
    let mut scd = Scd30::new_with_address(bus.acquire_i2c(), SCD30_ADDRESS);
    scd.start_measuring().unwrap();

    loop {
        // hprintln!("Pulling data from SCD30...");
        if let Ok(Some(measurement)) = scd.read() {
            // hprintln!("Read {:?}", measurement);

            // display content on the screen
            let mut buf_line1 = [0u8; 64];
            let mut buf_line2 = [0u8; 64];
            let line1: &str = stackfmt::fmt_truncate(
                &mut buf_line1,
                format_args!(
                    "T: {:.1} H: {:.1}%",
                    measurement.temperature, measurement.humidity
                ),
            );
            let line2: &str = stackfmt::fmt_truncate(
                &mut buf_line2,
                format_args!("CO2: {:.0}ppm", measurement.co2),
            );
            lcd.clear(&mut delay).unwrap();
            lcd.write_str(&line1, &mut delay).unwrap();
            lcd.set_cursor_pos(40, &mut delay).unwrap();
            lcd.write_str(&line2, &mut delay).unwrap();
        }
        delay.delay(2.secs());
    }
}
