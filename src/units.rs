use std::f64::consts::PI;

pub type Units = f64;

pub type Pt = (Units, Units);

pub type Distance = Units;

#[derive(Copy, Clone, Debug)]
pub struct Angle(Units);

impl Angle {
    pub fn new(value: Units) -> Self {
        Angle(value % (2.0 * PI))
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
        f64::cos(self.0)
    }

    pub fn cos_r(&self, r: Units) -> Units {
        self.cos() * r
    }

    pub fn sin(&self) -> Units {
        f64::sin(self.0)
    }

    pub fn sin_r(&self, r: Units) -> Units {
        self.sin() * r
    }

    pub fn value(&self) -> Units {
        self.0
    }

    pub fn turn() -> Self {
        Angle::new(2.0 * PI)
    }

    pub fn half_turn() -> Self {
        Angle::new(PI)
    }

    pub fn quarter_turn() -> Self {
        Angle::new(PI / 2.0)
    }
}
