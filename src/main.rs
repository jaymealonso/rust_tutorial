#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{self, repeat};
use std::io::{BufRead, BufReader, ErrorKind, Write};
// Added because couting of special chars is off
// https://stackoverflow.com/questions/46290655/does-rusts-string-have-a-method-that-returns-the-number-of-characters-rather-th
use unicode_segmentation::UnicodeSegmentation;

const MAX_EXERC: i32 = 25;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    // for a in args {
    //     dbg!(a);
    // }
    if args.is_empty() {
        println!(
            "Falta o numero do exercicio a ser impresso (1-{}).",
            MAX_EXERC
        );
        println!(" é possivel imprimir mais de um ao mesmo tempo indicando-os separados por espaço. (ex: 1 3 7)");
        return;
    }
    let mut number: i32;
    for arg in &args {
        // dbg!(&arg);
        number = arg.parse::<i32>().unwrap();
        _call_example(number);
    }
    // _call_example(number);
}

fn _call_example(number: i32) {
    println!();
    println!("===================================================================================================");
    println!("   Calling Example: {}", number);
    println!("===================================================================================================");

    match number {
        1 => ex01(),
        2 => ex02(),
        3 => ex03(),
        4 => ex04(),
        5 => ex05(),
        6 => ex06(),
        7 => ex07(),
        8 => ex08(),
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
        21 => ex21(),
        22 => ex22(),
        23 => ex23(),
        24 => ex24(),
        25 => ex25(),
        _ => println!("No example found!"),
    }
}

fn _print_ex_title(title: &str) {
    println!("---------------------------------------------------------------------------------------------------");
    print!("- {} ", title);
    let mut i = title.graphemes(true).count() + 3;
    while i < 98 {
        print!("-");
        i += 1;
    }
    println!("-");
    println!("---------------------------------------------------------------------------------------------------");
    println!();
}

fn ex01() {
    _print_ex_title("Pegando inputs e imprimindo eles");

    println!("What's your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("No Value");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn ex02() {
    _print_ex_title("Declarando variavel e verificando se é numero (expect)");

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "42";
    let mut age: u32 = age.trim().parse().expect("Age not a number.");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn ex03() {
    _print_ex_title("Imprimindo valores máximos.");

    println!("Max u32:   {}", u32::MAX);
    println!("Max u64:   {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128:  {}", u128::MAX);
    println!("Max f32:   {}", f32::MAX);
    println!("Max f64:   {}", f64::MAX);
}

fn ex04() {
    _print_ex_title("Somando floats 32 e 64.");

    let is_true: bool = true; //false
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
}

fn ex05() {
    _print_ex_title("Calculos com operações matematicas");

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 x 4 = {}", num_3 * num_4);
    println!("5 % 4 = {}", num_3 % num_4);
}

fn ex06() {
    _print_ex_title("Imprimindo numeros randomicos entre 1 e 100");

    ex06_print_rnd();
    ex06_print_rnd();
    ex06_print_rnd();
    ex06_print_rnd();
    ex06_print_rnd();
    ex06_print_rnd();
}

fn ex06_print_rnd() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random {}", random_num);
}

fn ex07() {
    _print_ex_title("Verificar se o aniversário é importante. (if else)");

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
    _print_ex_title("Pode votar ? (if inline)");

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
    _print_ex_title("Aniversário importante, (match)");

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
    _print_ex_title("Pode ou não votar, (valores com Ordering::Less/Ordering::Greater)");
    // let age = rand::thread_rng().gen_range(0..40);
    let age = 18;
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Age of {} can NOT vote!", age),
        Ordering::Greater | Ordering::Equal => println!("Age of {} can vote!", age),
    }
}

fn ex11() {
    _print_ex_title("Imprimindo valores com Loop (loop/break/continue)");

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
    _print_ex_title("(while)");

    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut arr_2_idx: usize = 0;
    while arr_2_idx < arr_2.len() {
        println!("Array: {}", arr_2[arr_2_idx]);
        arr_2_idx += 1;
    }
}

fn ex13() {
    _print_ex_title("(for)");

    let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for val in arr_2.iter() {
        println!("Array: {}", val);
    }
}

fn ex14() {
    _print_ex_title("(tuple)");

    let my_tuple: (u8, String, f64) = (42, "Jayme".to_string(), 50_000.00);

    println!("Name {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age {}", v1);
}

fn ex15() {
    _print_ex_title("replacing/pushing strings");

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let str2: String = st1.replace('A', "Another");
    println!("{}", str2);
}

fn ex16() {
    _print_ex_title("Vector sorting deleting duplicates");

    let st3: String = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Ramdom string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr: &[u8] = st5.as_bytes();
    println!("String length before [0..6]: {}", st5.len());
    let st6: &str = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st6: String = String::from("Just some");
    let st7: String = String::from(" words");
    let st8: String = st6 + &st7;
    for char in st8.bytes() {
        println!("{}: {}", char, char);
    }
}

fn ex17() {
    _print_ex_title("Casting");

    println!("------------------------------");
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", int3_u32)
}

fn ex18() {
    _print_ex_title("Enums Types");

    enum Day {
        Segunda,
        Terca,
        Quarta,
        Quinta,
        Sexta,
        Sabado,
        Domingo,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Sabado | Day::Domingo => true,
                _ => false,
            }
        }
    }
    let today: Day = Day::Segunda;
    println!("Is today the weeekend {}", today.is_weekend());
    println!("===================");
    println!("Mensagem do dia: ",);

    match today {
        Day::Segunda => println!("Todo mundo odeia segunda!"),
        Day::Terca => println!("Quase na metade"),
        Day::Quarta => println!("Meio da semana, pouco pra acabar"),
        Day::Quinta => println!("Amanhã é ponto facultativo, vem quem quer."),
        Day::Sexta => println!("#SEXTOU"),
        Day::Sabado => println!("FDS!"),
        Day::Domingo => println!("FDS!"),
    }
}

fn ex19() {
    _print_ex_title("Vectors");

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2st : {}", second),
        None => println!("No second value!"),
    }
}

fn ex20() {}
fn ex21() {}
fn ex22() {}
fn ex23() {}
fn ex24() {}
fn ex25() {}
