use arduino_hal::{
    hal::port::{PC0, PC1, PC2, PC3, PC4, PC5},
    port::{
        mode::{Analog, Floating, Input},
        Pin,
    },
    Adc,
};

pub struct Rng<'a> {
    a0: Pin<Analog, PC0>,
    a1: Pin<Analog, PC1>,
    a2: Pin<Analog, PC2>,
    a3: Pin<Analog, PC3>,
    a4: Pin<Analog, PC4>,
    a5: Pin<Analog, PC5>,
    adc: &'a mut Adc,
}

// Lifetime because we need to borrow the adc
impl<'a> Rng<'a> {
    pub fn new(
        a0: Pin<Input<Floating>, PC0>,
        a1: Pin<Input<Floating>, PC1>,
        a2: Pin<Input<Floating>, PC2>,
        a3: Pin<Input<Floating>, PC3>,
        a4: Pin<Input<Floating>, PC4>,
        a5: Pin<Input<Floating>, PC5>,
        adc: &'a mut Adc,
    ) -> Self {
        let a0 = a0.into_analog_input(adc);
        let a1 = a1.into_analog_input(adc);
        let a2 = a2.into_analog_input(adc);
        let a3 = a3.into_analog_input(adc);
        let a4 = a4.into_analog_input(adc);
        let a5 = a5.into_analog_input(adc);

        Self {
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            adc,
        }
    }

    pub fn bytes(&mut self) -> [u8; 32] {
        let mut rng_values: [u8; 36] = [0_u8; 36];

        for i in 0..6 {
            let analog = self.read_analog();
            rng_values[i * 6..i * 6 + 6].copy_from_slice(&analog);
            arduino_hal::delay_ms(100);
        }
        rng_values[0..32].try_into().unwrap()
    }

    pub fn read_analog(&mut self) -> [u8; 6] {
        [
            self.a0.analog_read(self.adc) as u8,
            self.a1.analog_read(self.adc) as u8,
            self.a2.analog_read(self.adc) as u8,
            self.a3.analog_read(self.adc) as u8,
            self.a4.analog_read(self.adc) as u8,
            self.a5.analog_read(self.adc) as u8,
        ]
    }
}
