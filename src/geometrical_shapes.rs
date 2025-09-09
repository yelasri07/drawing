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
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
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
    fn color() -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, Point::color());
    }

    fn color() -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        )
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = Line::color();
        draw_line(image, &self.a, &self.b, color);
    }

    fn color() -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        )
    }
}

fn draw_line(image: &mut Image, p1: &Point, p2: &Point, color: Color) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    // Handle the direction of the line, compute step sizes.
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    // Start at the initial point
    let mut x = p1.x as f32;
    let mut y = p1.y as f32;

    for _ in 0..steps {
        // Display the pixel at the current position
        image.display(x.round() as i32, y.round() as i32, color.clone());

        // Increment the position
        x += x_inc;
        y += y_inc;
    }
}
