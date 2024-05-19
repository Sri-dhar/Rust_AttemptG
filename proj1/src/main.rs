// use std::io;
//
// fn printtt(input: &str) {
//     let inputt = input.trim();
//     println!("{} is {}", inputt, inputt.len());
// }
//
// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     printtt(&input);
// }

// fn main()
// {
//     let x = 23;
//     let y : &str = "sridhar";
//     println!("{} {}", x, y);
// }
//

// fn main2()
// {
//     let x;
//     println!("x is: {}", x);
//     //gives error
// }

fn main22()
{
    let mut xx =(4,"sri");
    {
        let mut xx:(i32,&str) = (4234, "sridhar");
        let (x,y) = xx;
        println!("{} {}", x, y);
        xx.0 = 2345;
        xx.1 = "sridhar";

    }
    println!("{} {}", xx.0, xx.1);
}

fn main3()
{
    let mut x = {
        let y = 2354;
        y
    };
    println!("{}", x);
}

use std::cmp::min;

fn main4(){
    let least = min(3,66);
    println!("{}", least);
}
//
// fn main()
// {
//     let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
//     .iter()
//     .map(|x| x + 3)
//     .fold(0, |x, y| x + y);
//
//     println!("{}", x);
// }

//let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
//     .iter()
//     .map(|x| x + 3)
//     .fold(0, |x, y| x + y);

struct complex_no{
    real: f32,
    img: f32
}

use std::f32;

impl complex_no{
    fn new(real: f32, img: f32) -> complex_no{
        complex_no{real, img}
    }

    fn modulus(&self) -> f32 {
        f32::sqrt(self.real * self.real + self.img * self.img)
    }
}

fn main444()
{
    let x = complex_no{real : 3.00, img : 3.52};
    println!("Real part : {}\nImaginary Part:{}", x.real, x.img);
    let mut y = complex_no::new(3.00, 3.52);
    println!("Real part : {}\nImaginary Part:{}", y.real, y.img);

    let modulus : f32 = x.modulus();
    println!("Modulus: {}", modulus);

    // y.img = 32.3;
}

fn main8()
{
    let mut x = complex_no{
        real: 43.0,
        img: 345.0
    };

    panic!("hi , i am panicking");
    let y = complex_no::new(43.0, 345.0);
    println!("Real part : {}\nImaginary Part:{}", y.real, y.img);
}

// fn main() {
//     let x = plus_one(5);
//     plus_one(x);
//
//     println!("The value of x is: {x}");
// }
//
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
//


fn main12() {
    let v: Vec<i32> = (1..=10).collect();
    for i in &v {
        print!("{} ", i);
    }
}

fn main344()
{
    for i in vec![1,2,3,4,433,354]
    {
        println!("{}", i);
    }
}

fn main345() {
    let mut number = String::new();
    std::io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse()
        .expect("Please type a number!");

    if (number < 23 && number%2 == 0) {
        println!("Less than 23 and even");
    } else if (number > 23) {
        println!("Greater than 23");
    } else {
        println!("Equal to 23");
    }
}

fn mainWhile()
{
    let mut x = 0;
    while x<= 10{
        println!("{}", x);
        x += 1;
    }
}

fn maininfLoop()
{
    let mut x = 0;
    loop{
        println!("{}", x);
        x += 1;
        if x == 10{
            break;
        }
    }
}

fn mainforV2()
{
    for i in 0..34 {
        print!("{} ", i);
    }
}

fn mainforV3()
{
    for i in (0..34).rev() {
        print!("{} ", i);
    }
}

fn mainforV4()
{
    for i in (0..34).step_by(3) {
        print!("{} ", i);
    }
}

fn mainConditionV1()
{
    let mut condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);
}


fn mainwhile2() {
    let mut x = 0;
    while x <= 10 {
        println!("{}", x);
        x += 1;
    }
}

fn main() {

}