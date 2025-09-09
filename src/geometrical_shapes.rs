// src/geometrical_shapes.rs
use rand::prelude::*;
use raster::{Color, Image};

// Each shape must be drawn in a different color.
#[derive(Debug, Clone, Copy)]
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
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle {
            a: *a,
            b: *b,
            c: *c,
        }
    }
}

pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Rectangle {
        return Rectangle { a: *a, b: *b };
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

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = Line::color();
        draw_line(image, &self.a, &self.b, color.clone());
        draw_line(image, &self.b, &self.c, color.clone());
        draw_line(image, &self.c, &self.a, color.clone());
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

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = Line::color();
        draw_line(
            image,
            &self.a,
            &Point {
                x: self.b.x,
                y: self.a.y,
            },
            color.clone(),
        );
        draw_line(
            image,
            &Point {
                x: self.b.x,
                y: self.a.y,
            },
            &self.b,
            color.clone(),
        );
        draw_line(
            image,
            &self.b,
            &Point {
                x: self.a.x,
                y: self.b.y,
            },
            color.clone(),
        );
        draw_line(
            image,
            &Point {
                x: self.a.x,
                y: self.b.y,
            },
            &self.a,
            color.clone(),
        );
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

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = Circle::color();

        let xc = self.center.x;
        let yc = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = r;
        let mut d = 1 - r;

        while x <= y {
            image.display(xc + x, yc + y, color.clone());
            image.display(xc - x, yc + y, color.clone());
            image.display(xc + x, yc - y, color.clone());
            image.display(xc - x, yc - y, color.clone());
            image.display(xc + y, yc + x, color.clone());
            image.display(xc - y, yc + x, color.clone());
            image.display(xc + y, yc - x, color.clone());
            image.display(xc - y, yc - x, color.clone());

            x += 1;
            if d < 0 {
                d += 2 * x + 1;
            } else {
                y -= 1;
                d += 2 * (x - y) + 1;
            }
        }
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
