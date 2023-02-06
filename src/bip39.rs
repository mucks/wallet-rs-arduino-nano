use avr_progmem::{progmem, string::PmString};

use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PD0, PD1},
        Usart,
    },
    pac::USART0,
    port::{
        mode::{Input, Output},
        Pin,
    },
};

// #[link_section = ".progmem.data"]
// pub static WORD0_BYTES: [u8; 236] = *include_bytes!("../assets/WORDS0.txt");
// #[link_section = ".progmem.data"]
// pub static WORD1_BYTES: [u8; 236] = *include_bytes!("../assets/WORDS1.txt");
// #[link_section = ".progmem.data"]
// pub static WORD2_BYTES: [u8; 236] = *include_bytes!("../assets/WORDS2.txt");
// #[link_section = ".progmem.data"]
// pub static WORD3_BYTES: [u8; 236] = *include_bytes!("../assets/WORDS3.txt");

// pub fn index_to_bytes(index: usize) -> &'static [u8] {
//     match index {
//         0 => &WORD0_BYTES,
//         1 => &WORD1_BYTES,
//         2 => &WORD2_BYTES,
//         3 => &WORD3_BYTES,
//         _ => panic!("index out of bounds"),
//     }
// }

// pub fn index_to_word_bytes(index: usize) -> &'static [u8] {
//     match index {
//         0 => &WORD0_BYTES.split
//         1 => &WORD1_BYTES,
//         _ => panic!("index out of bounds"),
//     }
// }

pub fn get_mnemonic(indices: &[u16; 24]) -> [u8; 256] {
    let mut words_as_bytes = [32u8; 256];
    let mut j = 0;
    for (_i, word_index) in indices.iter().enumerate() {
        let len = get_word!(word_index).as_bytes().len();
        words_as_bytes[j..j + len].copy_from_slice(get_word!(word_index).as_bytes());
        j += len + 1;
    }
    words_as_bytes
}

pub fn print_mnnemonic(
    serial: &mut Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>,
    mm: &[u8; 256],
) {
    let mut word = [0u8; 16];
    let mut word_index = 1;
    let mut i = 0;
    for m in mm {
        word[i] = *m;
        if m == &32 {
            ufmt::uwrite!(serial, "{}: ", word_index).unwrap();
            for w in word {
                if w != 0 {
                    ufmt::uwrite!(serial, "{}", w as char).unwrap();
                }
            }
            ufmt::uwriteln!(serial, "").unwrap();
            word = [0u8; 16];
            i = 0;

            if word_index == 24 {
                break;
            }

            word_index += 1;
        }
        i += 1;
    }
}

macro_rules! words {
    ($($name:ident),*) => {
        progmem!{
        $(
            pub static progmem string $name = include_str!(concat!("../assets/", stringify!($name), ".txt"));
        )*
        }
    };
}

macro_rules! get_word {
    ($index:expr) => {{
        let words_index = ($index % 64) as usize;
        let word_index = ($index % 32) as usize;
        let words = index_to_words(words_index);
        words
            .load()
            .split_whitespace()
            .nth(word_index)
            .as_ref()
            .unwrap()
    }};
}

macro_rules! print_words {
    (&mut $serial:ident, $name:ident) => {{
        let buffer = $name.load();
        ufmt::uwriteln!($serial, "word: {}", &*buffer).unwrap();
    }};
}

pub(crate) use get_word;
pub(crate) use print_words;

words!(
    WORDS0, WORDS1, WORDS2, WORDS3, WORDS4, WORDS5, WORDS6, WORDS7, WORDS8, WORDS9, WORDS10,
    WORDS11, WORDS12, WORDS13, WORDS14, WORDS15, WORDS16, WORDS17, WORDS18, WORDS19, WORDS20,
    WORDS21, WORDS22, WORDS23, WORDS24, WORDS25, WORDS26, WORDS27, WORDS28, WORDS29, WORDS30,
    WORDS31, WORDS32, WORDS33, WORDS34, WORDS35, WORDS36, WORDS37, WORDS38, WORDS39, WORDS40,
    WORDS41, WORDS42, WORDS43, WORDS44, WORDS45, WORDS46, WORDS47, WORDS48, WORDS49, WORDS50,
    WORDS51, WORDS52, WORDS53, WORDS54, WORDS55, WORDS56, WORDS57, WORDS58, WORDS59, WORDS60,
    WORDS61, WORDS62, WORDS63
);

pub fn index_to_words(index: usize) -> PmString<236> {
    match index {
        0 => WORDS0,
        1 => WORDS1,
        2 => WORDS2,
        3 => WORDS3,
        4 => WORDS4,
        5 => WORDS5,
        6 => WORDS6,
        7 => WORDS7,
        8 => WORDS8,
        9 => WORDS9,
        10 => WORDS10,
        11 => WORDS11,
        12 => WORDS12,
        13 => WORDS13,
        14 => WORDS14,
        15 => WORDS15,
        16 => WORDS16,
        17 => WORDS17,
        18 => WORDS18,
        19 => WORDS19,
        20 => WORDS20,
        21 => WORDS21,
        22 => WORDS22,
        23 => WORDS23,
        24 => WORDS24,
        25 => WORDS25,
        26 => WORDS26,
        27 => WORDS27,
        28 => WORDS28,
        29 => WORDS29,
        30 => WORDS30,
        31 => WORDS31,
        32 => WORDS32,
        33 => WORDS33,
        34 => WORDS34,
        35 => WORDS35,
        36 => WORDS36,
        37 => WORDS37,
        38 => WORDS38,
        39 => WORDS39,
        40 => WORDS40,
        41 => WORDS41,
        42 => WORDS42,
        43 => WORDS43,
        44 => WORDS44,
        45 => WORDS45,
        46 => WORDS46,
        47 => WORDS47,
        48 => WORDS48,
        49 => WORDS49,
        50 => WORDS50,
        51 => WORDS51,
        52 => WORDS52,
        53 => WORDS53,
        54 => WORDS54,
        55 => WORDS55,
        56 => WORDS56,
        57 => WORDS57,
        58 => WORDS58,
        59 => WORDS59,
        60 => WORDS60,
        61 => WORDS61,
        62 => WORDS62,
        63 => WORDS63,
        _ => panic!("index out of range"),
    }
}
