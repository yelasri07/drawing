use rand::prelude::*;
use raster::{Color, Image};

// Each shape must be drawn in a different color.
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn random(width: i32, height: i32) -> Point {
        let mut rng = rand::thread_rng();
        Point { x: rng.gen_range(0..width), y: rng.gen_range(0..height) }
    }
}

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        return Line { a: a, b: b };
    }
    pub fn random(width: i32, heigh: i32) -> Line {
        let mut rng = rand::thread_rng();
        Line {
            a: Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..heigh),
            },
            b: Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..heigh),
            },
        }
    }
}

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        return Triangle { a: a, b: b, c: c };
    }
}

pub struct Rectangle {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

impl Rectangle {
    pub fn new(a: Point, b: Point, c: Point, d: Point) -> Rectangle {
        return Rectangle {
            a: a,
            b: b,
            c: c,
            d: d,
        };
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Circle {
        return Circle {
            center: center,
            radius: radius,
        };
    }

    pub fn random(width: i32, heigh: i32) -> Circle {
        let mut rng = rand::thread_rng();
        Circle {
            center: Point {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..heigh),
            },
            radius: rng.gen_range(0..(width + heigh) / 2),
        }
    }
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color();
}

pub trait Displayable {
    fn display() {}
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.x, self.y, Color { r: 255, g: 255, b: 255, a: 255 }).unwrap()
    }

    fn color() {
        
    }
}