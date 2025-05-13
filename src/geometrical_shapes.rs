use rand::prelude::*;
use raster::{Color, Image};
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color::rgb(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        )
    }
}

pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(rng.random_range(0..width), rng.random_range(0..height))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.0, self.1, self.color());
    }
}



// pub struct Line(Point, Point);

// impl Line {
//     pub fn new(p1: Point, p2: Point) -> Self {
//         Line(p1, p2)
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let mut rng = rand::rng();
//         Self::new(&Point::random(width, height), &Point::random(width, height))
//     }
// }

// pub struct Triangle(Point, Point, Point);

// impl Triangle {
//     pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
//         Triangle(p1, p2, p3)
//     }
// }