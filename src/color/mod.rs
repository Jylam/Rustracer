use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::{self, Formatter, Display};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    // Constructor
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color{r: r, g: g, b: b}
    }

    // Accessors
    pub fn r(&self) -> f64 {
        self.r
    }
    pub fn g(&self) -> f64 {
        self.g
    }
    pub fn b(&self) -> f64 {
        self.b
    }

}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.r, self.g, self.b)
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, v2: Color) -> Color {
        Color {r: self.r + v2.r, g: self.g + v2.g, b: self.b + v2.b}
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, v2: Color) -> Color {
        Color {r: self.r - v2.r, g: self.g - v2.g, b: self.b - v2.b}
    }
}
impl Mul for Color {
    type Output = Color;
    fn mul(self, v2: Color) -> Color {
        Color {r: self.r * v2.r, g: self.g * v2.g, b: self.b * v2.b}
    }
}
impl Mul<i32> for Color {
    type Output = Color;
    fn mul(self, t: i32) -> Color {
        Color {r: self.r * t as f64, g: self.g * t as f64, b: self.b * t as f64}
    }
}
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, t: f64) -> Color {
        Color {r: self.r * t, g: self.g * t, b: self.b * t}
    }
}
impl Div<i32> for Color {
    type Output = Color;
    fn div(self, t: i32) -> Color {
        Color {r: self.r / t as f64, g: self.g / t as f64, b: self.b / t as f64}
    }
}
impl Div<f64> for Color {
    type Output = Color;
    fn div(self, t: f64) -> Color {
        Color {r: self.r / t, g: self.g / t, b: self.b / t}
    }
}



