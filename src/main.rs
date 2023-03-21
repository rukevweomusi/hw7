#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
fn main() {
    let rect = Shape::Rectangle{width: 10.0, height: 5.0};
    let triangle = Shape::Triangle{base: 10.0, height: 5.0, side_1: 5.0, side_2: 5.0, side_3: 5.0};
    let circle = Shape::Circle{radius: 5.0};
    println!("Triangle area = {}", triangle.area());
    println!("verify rectangle: {}", rect.verifyShape());
    //println!("verify triangle: {}", triangle.verifyShape());
    println!("verify circle: {}", circle.verifyShape());
}
enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Triangle { base: f64, height: f64, side_1: f64, side_2: f64, side_3: f64},
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => 
                height*width,
            Shape::Circle { radius } => 
                3.14 * radius.powi(2),
            Shape::Triangle { base, height,.. } => 
                0.5 * base * height,
        }
    }


//double
fn double(&self) -> f64 {
    match self {
        Shape::Rectangle { width, height } => 
            (2.0 * width) * (2.0 * height),
        Shape::Circle { radius } => 
            3.14 * (2.0 * radius),
        Shape::Triangle { base, height,.. } => 
            0.5 * 2.0*base * 2.0*height
    }
}

fn perimeter(&self) -> f64 {
    match self {
        Shape::Rectangle { width, height } => 
            (2.0 * width) + (2.0 * height),
        Shape::Circle { radius } => 
            2.0 * 3.14 * radius,
        Shape::Triangle { side_1, side_2, side_3,.. } => 
            side_1 + side_2 + side_3,
    }
}
fn verifyShape(&self) -> &str {
    match self {
        Shape::Rectangle { width, height } if height > &0.0 && width > &0.0=> 
            return "Valid rectangle",
        Shape::Circle { radius } if radius > &0.0 => 
            return "Valid circle",
        Shape::Triangle { base, height, side_1, side_2, side_3 } if (base > &0.0 && height>&0.0 && side_1>&0.0 && side_2 >&0.0 && side_3>&0.0) && ((side_1 + side_2 > *side_3 || (side_1 + side_3 > *side_2) || side_2 + side_3 > *side_1))=> 
            return "Valid triangle",
        &_ => return "Invalid shape"
    }
}
}