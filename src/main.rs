#![no_std]
#![no_main]

mod bip39;
mod rng;

use arduino_hal::Adc;

use bip39::*;
use rng::Rng;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    if let Some(s) = info.payload().downcast_ref::<&str>() {
        ufmt::uwriteln!(serial, "panic occurred: {}", s).unwrap();
    } else {
        ufmt::uwriteln!(serial, "panic occurred").unwrap();
    }

    loop {}
}

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

    let t: u8 = 237;

    ufmt::uwriteln!(&mut serial, "words_index: {}", get_word!(t)).unwrap();

    loop {
        let bytes = rng.bytes();
        ufmt::uwriteln!(&mut serial, "{:?}", bytes).unwrap();
        arduino_hal::delay_ms(100);
    }
}
