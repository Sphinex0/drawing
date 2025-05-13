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

#[derive(Debug, Clone)]
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



pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line(p1.clone(), p2.clone())
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x1 = self.0.0 ;
        let x2 = self.1.0 ;
        let y1 = self.0.1 ;
        let y2 = self.1.1 ;
        let x_diff = x2-x1;
        let y_diff = y2-y1;

        let max:i32 = x_diff.max(y_diff);
        let mut current_x = x1.into();
        let mut current_y= y1.into();
        
        for _ in 0..max{
            image.display(current_x, current_y, self.color());
            current_x += x_diff/max;
            current_y += y_diff/max;
        }

        //image.display(self.0, self.1, self.color());
    }
}

// pub struct Triangle(Point, Point, Point);

// impl Triangle {
//     pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
//         Triangle(p1, p2, p3)
//     }
// }