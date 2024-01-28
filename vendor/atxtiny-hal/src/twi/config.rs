use crate::time::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// TWI bus clock frequency
    pub frequency: Hertz,
    /// SCL and SDA rise time
    pub rise_time: NanosDuration,
    /// Fast Mode Plus enabled (400kHz to 1Mhz)
    pub fast_mode_plus: bool,
}

impl Config {
    /// Sets the given TWI clock frequency.
    pub fn frequency(mut self, frequency: impl Into<Hertz>) -> Self {
        self.frequency = frequency.into();
        self
    }

    /// Sets the given rise time.
    pub fn rise_time(mut self, rise_time: NanosDuration) -> Self {
        self.rise_time = rise_time.into();
        self
    }

    pub fn default_for_frequency(frequency: impl Into<Hertz>) -> Option<Self> {
        // Maximum rise-times according to datasheet:
        // fscl <= 100KHz  -> trise = 1000ns
        // fscl <= 400KHz  -> trise =  300ns
        // fscl <= 1000KHz -> trise =  120ns
        let frequency = frequency.into();

        if frequency.raw() <= 100_000 {
            Some(Self { frequency, rise_time: 1000.nanos(), fast_mode_plus: false })
        } else if frequency.raw() <= 400_000 {
            Some(Self { frequency, rise_time: 300.nanos(), fast_mode_plus: false })
        } else if frequency.raw() <= 1_000_000 {
            Some(Self { frequency, rise_time: 120.nanos(), fast_mode_plus: true })
        } else {
            None
        }
    }
}

impl Default for Config {
    /// Creates a new configuration with typically used parameters for a clock
    /// rate of 100KHz
    fn default() -> Config {
        Config::default_for_frequency(100_000.Hz()).unwrap()
    }
}

// NOTE: cannot implement TryFrom because of blanket implementation in core
impl<F: Into<Hertz>> From<F> for Config {
   fn from(f: F) -> Config {
       Config::default_for_frequency(f).unwrap()
   }
}
