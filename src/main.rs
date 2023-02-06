#![no_std]
#![no_main]

mod bip39;
mod rng;

use arduino_hal::{eeprom, Adc};

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
    let mut eeprom = eeprom::Eeprom::new(dp.EEPROM);
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut rng = Rng::new(
        pins.a0, pins.a1, pins.a2, pins.a3, pins.a4, pins.a5, &mut adc,
    );

    ufmt::uwriteln!(&mut serial, "Start").unwrap();

    //eeprom.read(0, &mut buffer).unwrap();
    ufmt::uwriteln!(&mut serial, "Generating new mnemonic...").unwrap();

    let indices = rng.indices();
    let mm = get_mnemonic(&indices);
    print_mnnemonic(&mut serial, &mm);

    //eeprom.write(0, &mm).unwrap();

    // unsafe {
    //     let s = core::str::from_utf8_unchecked(&mm);
    //     ufmt::uwriteln!(&mut serial, "mnemonic: {}", s).unwrap();
    // }

    loop {
        //ufmt::uwriteln!(&mut serial, "words_index: {}", get_word!(indices[0])).unwrap();
        arduino_hal::delay_ms(100);
    }
}
