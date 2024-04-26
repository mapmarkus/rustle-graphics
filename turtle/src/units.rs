use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

const HALF_PI: f64 = PI / 2.0;

pub type Units = f64;

pub type Pt = (Units, Units);

pub type Distance = Units;

pub type Thickness = Units;

/// Angle "turn" representation.
///
/// Its interval is [-2*PI, 2*PI]. Angles that fall outside of the range are normalised.
///
/// NOTE: +/- 2*PI is not normalised.
///
#[derive(Copy, Clone, Debug)]
pub struct Angle(Units);

impl Angle {
    pub fn new(value: Units) -> Self {
        if value.abs() > TWO_PI {
            Angle(value % TWO_PI)
        } else {
            Angle(value)
        }
    }

    pub fn negate(&self) -> Self {
        Angle::new(-self.0)
    }

    pub fn add(&self, other: Angle) -> Self {
        Angle::new(self.0 + other.0)
    }

    pub fn subtract(&self, other: Angle) -> Self {
        Angle::new(self.0 - other.0)
    }

    pub fn cos(&self) -> Units {
        Units::cos(self.0)
    }

    pub fn cos_r(&self, r: Units) -> Units {
        self.cos() * r
    }

    pub fn sin(&self) -> Units {
        Units::sin(self.0)
    }

    pub fn sin_r(&self, r: Units) -> Units {
        self.sin() * r
    }

    pub fn value(&self) -> Units {
        self.0
    }

    pub fn turn() -> Self {
        Angle::new(TWO_PI)
    }

    pub fn half_turn() -> Self {
        Angle::new(PI)
    }

    pub fn quarter_turn() -> Self {
        Angle::new(HALF_PI)
    }

    pub fn zero() -> Self {
        Angle::new(0.0)
    }
}
