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
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::rng();
        Self::new(rng.random_range(0..width), rng.random_range(0..height))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

pub struct Line {
    p1: Point,
    p2: Point,
    color: Color,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut line = Self {
            p1: p1.clone(),
            p2: p2.clone(),
            color: Color::white(),
        };
        line.color = line.color();
        line
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let x1 = self.p1.x;
        let x2 = self.p2.x;
        let y1 = self.p1.y;
        let y2 = self.p2.y;
        let x_diff = (x2 - x1).abs();
        let y_diff = (y2 - y1).abs();

        let max: i32 = x_diff.max(y_diff);
        let mut current_x: f64 = x1.into();
        let mut current_y: f64 = y1.into();
        for _ in 0..max {
            image.display(current_x as i32, current_y as i32, self.color.clone());
            if x1 < x2 {
                current_x += x_diff as f64 / max as f64;
            } else {
                current_x -= x_diff as f64 / max as f64;
            }

            if y1 < y2 {
                current_y += y_diff as f64 / max as f64;
            } else {
                current_y -= y_diff as f64 / max as f64;
            }
        }
    }
}

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            a: p1.clone(),
            b: p2.clone(),
            c: p3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut line1 = Line::new(&self.a, &self.b);
        let mut line2 = Line::new(&self.b, &self.c);
        let mut line3 = Line::new(&self.c, &self.a);
        line1.color = color.clone();
        line2.color = color.clone();
        line3.color = color.clone();

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }
}

pub struct Rectangle {
    left_p: Point,
    right_p: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            left_p: p1.clone(),
            right_p: p2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();

        let point1 = &Point::new(self.left_p.x, self.right_p.y);
        let point2 = &Point::new(self.right_p.x, self.left_p.y);
        let mut line1 = Line::new(&self.left_p, point1);
        let mut line2 = Line::new(point1, &self.right_p);
        let mut line3 = Line::new(&self.right_p, point2);
        let mut line4 = Line::new(point2, &self.left_p);

        line1.color = color.clone();
        line2.color = color.clone();
        line3.color = color.clone();
        line4.color = color.clone();

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}
impl Circle {
    pub fn new(p1: &Point, r: i32) -> Self {
        Circle {
            center: p1.clone(),
            radius: r,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(
            &Point::random(width, height),
            rand::rng().random_range(0..width.min(height)),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let angle = 0f64;
        let mut last_x = ((self.radius as f64 * angle.cos() as f64) + self.center.x as f64) as i32;
        let mut last_y = ((self.radius as f64 * angle.sin() as f64) + self.center.y as f64) as i32;
        let color = self.color();

        for i in 0..=360 {
            let last_point = Point::new(last_x, last_y);
            let angle = (i as f64) * std::f64::consts::PI / 180.0;
            let x = ((self.radius as f64 * angle.cos() as f64) + self.center.x as f64) as i32;
            let y = ((self.radius as f64 * angle.sin() as f64) + self.center.y as f64) as i32;
            last_x = x;
            last_y = y;

            // image.display(x, y, color.clone());
            let mut line = Line::new(&last_point, &Point::new(x, y));
            line.color = color.clone();
            line.draw(image);
        }
    }
}
