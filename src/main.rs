mod list_testcase;
mod formatting;

use list_testcase::List;
use formatting::Color;
use std::fmt;
use std::fmt::{write, Formatter};

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "Structure({})", self.0)
    }
    
}

#[derive(Debug)]
struct Print2D{
    a: f64,
    b: f64
}

impl fmt::Display for Print2D{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Print2D({}, {})", self.a, self.b)
    }
}


#[derive(Debug)]
struct Complex{
    re: f64,
    im: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}i", self.re, self.im)
    }
}



fn main(){
    println!("Hello World!");
    println!("I'm a Rustacean!");
    let x = 5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("My name is {0}, {1} {0}","James","Bond");
    let structure = Structure(3);
    println!("Display: {}", structure);
    println!("Debug: {:?}", structure);

    let print2d = Print2D{a: 3.2, b: 4.2};
    println!("Display: {}", print2d);
    println!("Debug: {:?}", print2d);

    let complex = Complex{re : 3.3, im:4.3};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);


    let v = List(vec![1,2,3]);
    println!("Display: {}", v);


    let color = Color{red:128, green:255, blue:90};
    let color2 = Color{red:0, green:3, blue:254};
    println!("Display: {}", color);
    println!("Display: {}", color2);
}
