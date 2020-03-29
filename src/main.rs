#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    pac,
    prelude::*,

};

use stm32f1xx_hal::delay::Delay;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();

    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    // Prepare the GPIOC peripheral
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        led.set_low().unwrap();
        delay.delay_ms(1000 as u32);
        led.set_high().unwrap();
        delay.delay_ms(1000 as u32);
    }
}
