#![no_std]
#![no_main]

mod rng;

use arduino_hal::Adc;
use panic_halt as _;
use rng::Rng;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut adc = Adc::new(dp.ADC, Default::default());
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut rng = Rng::new(
        pins.a0, pins.a1, pins.a2, pins.a3, pins.a4, pins.a5, &mut adc,
    );

    ufmt::uwriteln!(&mut serial, "Start").unwrap();

    loop {
        let bytes = rng.bytes();
        ufmt::uwriteln!(&mut serial, "{:?}", bytes).unwrap();
        arduino_hal::delay_ms(100);
    }
}
