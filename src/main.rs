#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Direct);

    let mut led = pins.d8.into_output().into_pwm(&timer4);
    led.enable();

    loop {
        for brightness in 0..=255 {
            led.set_duty(brightness);
            arduino_hal::delay_ms(15);
        }
        for brightness in (0..=255).rev() {
            led.set_duty(brightness);
            arduino_hal::delay_ms(15);
        }
    }
}
