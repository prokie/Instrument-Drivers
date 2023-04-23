//! This crate contains instrument drivers for various instruments.
pub mod keysight;

/// A trait for power supplies
pub trait PowerSupply {
    /// Set the voltage of the power supply channel
    fn set_voltage(&mut self, voltage: f64);
    /// Set the current of the power supply channel
    fn set_current(&mut self, current: f64);
    /// Set the output of the power supply channel
    fn set_output(&mut self, output: bool);
    /// Set the slew rate of the power supply channel
    fn set_slew_rate(&mut self, slew_rate: f64);
    /// Get the voltage of the power supply channel
    fn get_voltage(&self) -> f64;
    /// Get the current of the power supply channel
    fn get_current(&self) -> f64;
    /// Get the output of the power supply channel
    fn get_output(&self) -> bool;
}

/// A trait for signal analyzers
pub trait SignalAnalyzer {
    /// Set the center frequency of the signal analyzer
    fn set_center_frequency(&mut self, center_frequency: f64);
    /// Set the span of the signal analyzer
    fn set_span(&mut self, span: f64);
    /// Set the resolution bandwidth of the signal analyzer
    fn set_resolution_bandwidth(&mut self, resolution_bandwidth: f64);
    /// Set the video bandwidth of the signal analyzer
    fn set_video_bandwidth(&mut self, video_bandwidth: f64);
}

/// A trait for multimeters
pub trait Multimeter {
    /// Measure the DC current
    fn measure_dc_current(&self) -> f64;
    /// Measure the DC voltage
    fn measure_dc_voltage(&self) -> f64;
}

/// A trait for oscilloscopes
pub trait Oscilloscope {
    /// Set the time scale of the oscilloscope
    fn set_time_scale(&mut self, time_scale: f64);
    /// Set the reference clock of the oscilloscope
    fn set_reference_clock(&mut self, reference_clock: f64);
}

pub struct Instrument {
    pub resource_manager: visa_rs::DefaultRM,
}
