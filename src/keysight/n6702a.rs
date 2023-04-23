/// A Keysight N6702A power supply
pub struct N6702a {
    pub channels: Vec<Channel>,
    pub status: Status,
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
impl Default for N6702a {
    fn default() -> Self {
        Self::new()
    }
}
impl N6702a {
    pub fn new() -> N6702a {
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
        }
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
    use crate::PowerSupply;

    use super::*;

    #[test]
    fn it_works() {
        let mut psu_a = N6702a::new();
        psu_a.channels[0].set_voltage(1.0);
        assert_eq!(psu_a.channels[0].get_voltage(), 1.0);
    }
}
