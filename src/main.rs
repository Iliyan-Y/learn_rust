use std::io;

fn main() {
    get_input();
    variables();
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
