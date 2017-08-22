use std::f32::consts::PI;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
pub struct Circle {
    pub diameter: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        let radius: f32 = self.diameter / 2.0;
        PI * ( radius * radius )
    }
}
