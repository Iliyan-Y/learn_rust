use std::{cmp::Ordering, io};

use rand::Rng;
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // get_input();
    // variables();
    // data_types();
    // match_condition();
    // arrays();
    //enums();

    let im_string = "string from".to_string();

    order_food();
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

fn match_condition() {
    let age2 = rand::thread_rng().gen_range(1..100);
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important birthday"),
        65..=i32::MAX => println!("Important birthday"),
        _ => println!("NOT IMPORTANT BD"), // everything else, required for match statement
    };

    let my_age = rand::thread_rng().gen_range(1..22);
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        std::cmp::Ordering::Greater => println!("Can vote"),
        std::cmp::Ordering::Equal => println!("Can vote for first time"),
    }
}

fn arrays() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("Length : {}", arr.len());
    let mut arr_index = 0;

    loop {
        if arr[arr_index] % 2 == 0 {
            arr_index += 1;
            continue;
        };

        if arr[arr_index] == 7 {
            break;
        }

        println!("Val {}", arr[arr_index]);
        arr_index += 1
    }

    arr_index = 0;
    while arr_index < arr.len() {
        print!("arr loop");
        arr_index += 1
    }

    for val in arr.iter() {
        print!("\nfor loop {}", val);
    }
}

fn enums() {
    enum Days {
        Monday,
        _Tuesday,
        _Wednesday,
        _Thursday,
        _Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Sunday;

    println!("Is today weekend ? {}", today.is_weekend());
}
