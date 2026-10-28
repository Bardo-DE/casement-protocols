//! Output value types: scale and refresh as integers, so the model is `Eq`.

/// A scale factor in thousandths (1500 is 1.5): `wl_fixed` as an integer, so it is `Eq`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScaleMilli(u32);

impl ScaleMilli {
    /// Scale 1.
    pub const ONE: ScaleMilli = ScaleMilli(1000);

    /// The scale `milli` thousandths.
    pub const fn new(milli: u32) -> Self {
        Self(milli)
    }

    /// The thousandths.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// The scale nearest `value`, a `wl_fixed` already read as a float; negative and NaN
    /// become 0.
    pub fn from_f64(value: f64) -> Self {
        Self((value * 1000.0).round().max(0.0) as u32)
    }

    /// The scale as the float `wl_fixed` carries.
    pub fn to_f64(self) -> f64 {
        f64::from(self.0) / 1000.0
    }
}

/// A refresh rate in millihertz (60000 is 60 Hz), as the protocol carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefreshMilliHz(u32);

impl RefreshMilliHz {
    /// The rate `milli_hz`.
    pub const fn new(milli_hz: u32) -> Self {
        Self(milli_hz)
    }

    /// The millihertz.
    pub const fn get(self) -> u32 {
        self.0
    }
}
