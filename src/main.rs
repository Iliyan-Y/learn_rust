use std::io;

use rand::{random, Rng};

fn main() {
    // get_input();
    // variables();
    data_types();
}

fn get_input() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "nice to meet you";

    io::stdin().read_line(&mut name).expect("error ");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn variables() {
    const ONE_MILE: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("age wsn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MILE);
}

fn data_types() {
    // Unsigned integer (can be only zero or positive) : u8 u16 u32 u64 u128, usize
    // Signed integer (can be + 0 -): i8 i16 i32 i64 i128, isize

    // println!("Max u32: {}", u32::MAX);
    // println!("Max u64: {}", u64::MAX);
    // println!("Max usize: {}", usize::MAX);
    // println!("Max u128: {}", u128::MAX);
    // println!("Max f32: {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);

    // underscore in front of variable ignore unused warning
    let _is_true: bool = true;

    //Math

    let random_number: i32 = rand::thread_rng().gen_range(1..1000);
    let random_number_2: i32 = rand::thread_rng().gen_range(1..1000);
    println!(
        "MATH RANDOM: {}, + {} = {}",
        random_number,
        random_number_2,
        random_number + random_number_2
    )
}
