use std::io::{BufRead, BufReader, Read, Write};

/// A Keysight N6702A power supply
pub struct N6702a {
    pub channels: Vec<Channel>,
    pub status: Status,
    pub instrument: visa_rs::Instrument,
}

/// The status of the power supply
/// It can be either `ON` or `OFF´
pub enum Status {
    On,
    Off,
}

/// Represents a channel that can be controlled by a power supply.
///
/// Each channel has a unique `channel_number` that identifies it, and can be turned on or off using
/// the `output` field. The `voltage` and `current` fields represent the voltage and current levels
/// of the channel, respectively.
pub struct Channel {
    /// The channel's unique identifier.
    pub channel_number: usize,
    /// The voltage level of the channel.
    pub voltage: f64,
    /// The current level of the channel.
    pub current: f64,
    /// Whether the channel's output is currently enabled.
    pub output: bool,
}

impl N6702a {
    pub fn new(instrument: visa_rs::Instrument) -> N6702a {
        let mut channels = Vec::new();
        for i in 1..=4 {
            channels.push(Channel {
                channel_number: i,
                voltage: 0.0,
                current: 0.0,
                output: false,
            });
        }
        N6702a {
            channels,
            status: Status::Off,
            instrument,
        }
    }
    pub fn get_identification(&mut self) -> String {
        self.instrument.write_all(b"*IDN?").unwrap();
        let mut buf_reader = BufReader::new(&self.instrument);
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).unwrap().to_string()
    }
}

impl crate::PowerSupply for Channel {
    fn set_voltage(&mut self, voltage: f64) {
        todo!()
    }
    fn set_current(&mut self, current: f64) {
        todo!()
    }
    fn set_output(&mut self, output: bool) {
        todo!()
    }
    fn set_slew_rate(&mut self, slew_rate: f64) {
        todo!()
    }
    fn get_voltage(&self) -> f64 {
        todo!()
    }
    fn get_current(&self) -> f64 {
        todo!()
    }
    fn get_output(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use visa_rs::{flags::AccessMode, DefaultRM, TIMEOUT_IMMEDIATE};

    #[test]
    fn it_works() {
        let rm = DefaultRM::new().unwrap(); //open default resource manager
        let expr = CString::new("?*KEYSIGH?*INSTR").unwrap().into(); //expr used to match resource name
        let rsc = rm.find_res(&expr).unwrap(); // find the first resource matched
        let instr = rm
            .open(&rsc, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
            .unwrap();

        let mut n6702a = N6702a::new(instr);

        assert_eq!(
            n6702a.get_identification(),
            "Keysight Technologies,N6702C,MY56004610,E.02.07.3231"
        );
    }
}
