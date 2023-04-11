use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Write 1 to generait for first half");
    println!("Write 2 to generait for second half");
    println!("Write 12 to generait for both");

    let pr_skupina: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let dr_skupina = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let halva:Vec<i32> = vec![1, 2];
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let halvni = rand::thread_rng().gen_range(0..=1);
    let sluzba_1 = rand::thread_rng().gen_range(0..13);
    let sluzba_2 = rand::thread_rng().gen_range(0..14);


    println!("Hlava otrocke operace 1.C je Quatarsky sheik Adam Tybl");
    match guess {
        1 => println!("{}", pr_skupina[sluzba_1]),
        2 => println!("{}", dr_skupina[sluzba_2]),
        12 => println!("{}, {}  \
        {}", pr_skupina[sluzba_1], halva[halvni], dr_skupina[sluzba_2]),
        _ => println!("Ty jsi oplili obcane"),
    }
}