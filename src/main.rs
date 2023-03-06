#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, repeat};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // for a in args {
    //     dbg!(a);
    // }
    // let number: i32 = args[1].parse::<i32>().unwrap();
    // let arg: char = Some(args.get(1));
    // let number = arg.parse::<u64>();
    //
    // match number {
    //     1 => ex01(),
    //     2 => ex02(),
    //     3 => ex03(),
    //     4 => ex05(),
    //     5 => ex06(),
    //     6 => ex07(),
    //     7 => ex08(),
    //     8 => ex09(),
    //     _ => println!("No example found!"),
    // }
    // let exercicio: fn() = format!("ex{}", number);
    // exercicio();
    // ex13();
    _call_example(15);
}

fn _call_example(number: i32) {
    println!();
    println!("===================================================================================================");
    println!("   Calling Example: {}", number);
    println!("===================================================================================================");
    println!();

    match number {
        1 => ex01(),
        2 => ex02(),
        3 => ex03(),
        4 => ex05(),
        5 => ex06(),
        6 => ex07(),
        7 => ex08(),
        8 => ex09(),
        9 => ex09(),
        10 => ex10(),
        11 => ex11(),
        12 => ex12(),
        13 => ex13(),
        14 => ex14(),
        15 => ex15(),
        16 => ex16(),
        17 => ex17(),
        18 => ex18(),
        19 => ex19(),
        20 => ex20(),
        _ => println!("No example found!"),
    }
}

fn ex01() {
    println!("What's your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("No Value");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn ex02() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "42";
    let mut age: u32 = age.trim().parse().expect("Age not a number.");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn ex03() {
    println!("Max u32:   {}", u32::MAX);
    println!("Max u64:   {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128:  {}", u128::MAX);
    println!("Max f32:   {}", f32::MAX);
    println!("Max f64:   {}", f64::MAX);
}

fn ex04() {
    let is_true: bool = true; //false
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
}

fn ex05() {
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 x 4 = {}", num_3 * num_4);
    println!("5 % 4 = {}", num_3 % num_4);
}

fn ex06() {
    _print_rnd();
    _print_rnd();
    _print_rnd();
    _print_rnd();
    _print_rnd();
    _print_rnd();
}

fn _print_rnd() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random {}", random_num);
}

fn ex07() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important birthday!")
    } else if (age == 21) || (age == 50) {
        println!("Important birthday XXXX!")
    } else if age >= 65 {
        println!("Important birthday! xxxx")
    } else {
        println!("Not important birthday!")
    }
}

fn ex08() {
    // let mut my_age = 42;
    let mut my_age = rand::thread_rng().gen_range(10..40);
    let can_vote: bool = my_age > 18;
    println!(
        "Idade de {}{} pode votar.",
        my_age,
        if !can_vote { " não" } else { "" }
    )
}

fn ex09() {
    let mut age = rand::thread_rng().gen_range(8..80);
    print!("Age of {} is an ", age);
    match age {
        1..=18 => println!("Important birthday!"),
        21 | 50 => println!("Important birthday! XXXX"),
        65..=i32::MAX => println!("Important birthday! XXXX"),
        _ => println!("Not important birthday!"),
    }
}

fn ex10() {
    // let age = rand::thread_rng().gen_range(0..40);
    let age = 18;
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Age of {} can NOT vote!", age),
        Ordering::Greater | Ordering::Equal => println!("Age of {} can vote!", age),
    }
}

fn ex11() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());
    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut arr_2_idx: usize = 0;
    loop {
        if (arr_2[arr_2_idx] % 2 == 0) {
            arr_2_idx += 1;
            continue;
        }
        if (arr_2[arr_2_idx] == 9) {
            break;
        }
        println!("Val: {}", arr_2[arr_2_idx]);
        arr_2_idx += 1;
    }
}

fn ex12() {
    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut arr_2_idx: usize = 0;
    while arr_2_idx < arr_2.len() {
        println!("Array: {}", arr_2[arr_2_idx]);
        arr_2_idx += 1;
    }
}

fn ex13() {
    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for val in arr_2.iter() {
        println!("Array: {}", val);
    }
}

fn ex14() {
    let my_tuple: (u8, String, f64) = (42, "Jayme".to_string(), 50_000.00);

    println!("Name {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age {}", v1);
}

fn ex15() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let str2: String = st1.replace('A', "Another");
    println!("{}", str2);
}
fn ex16() {}
fn ex17() {}
fn ex18() {}
fn ex19() {}
fn ex20() {}
