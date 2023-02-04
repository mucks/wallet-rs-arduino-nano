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

    let index = 237;

    let words_index = index % 64;
    let word_index = (index % 32) as usize;
    let words = index_to_words(words_index as usize);

    let word_in_memory = words.load().as_bytes();

    // let word = get_word!(237).as_bytes();
    // let word2 = get_word!(238).as_bytes();

    // let x = word.get(0);

    // let mut bytes: [u8; 16] = [0; 16];
    //bytes[0..5].copy_from_slice(word);

    // let word2 = get_word!(238);

    // let words = [word, word2];

    let t: u8 = 237;

    ufmt::uwriteln!(&mut serial, "words_index: {}", get_word!(t)).unwrap();

    loop {
        let bytes = rng.bytes();
        ufmt::uwriteln!(&mut serial, "{:?}", bytes).unwrap();
        arduino_hal::delay_ms(100);
    }
}
