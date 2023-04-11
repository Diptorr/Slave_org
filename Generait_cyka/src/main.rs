use std::io;
use rand::Rng;

fn main() {
    println!("Write 0 to generait for first half");
    println!("Write 2 to generait for second half");
    println!("Write 11 to generait for both");

    let pr_skupina:[&str; 13]  = ["Kopejs", "Dan", "Hladik"," Stastny"," Kozma"," Ilija"," Karel"," Sauer"," Patrik"," Borovicka"," Julian"," Lucie"," Kamen"];
    let dr_skupina:[&str; 14] = ["Terka"," Jasmin"," Davca"," Zikmund"," Hana"," Lorenc"," Lenka"," Matous"," Ondra"," Pavel"," Rene"," Terka"," Vera"," Tybl"];
    let halva:[&str; 2] = ["Hlava", "Podradny"];
    let mut intp = String::new();
    let mut intp_2 =String::new();

    io::stdin().read_line(&mut intp).expect("Failed to read line");
    println!("Chtel by jsi i hlavni a vedlejsi roly?
                        Jestli ano      zamackni 1
                        Jestli ne       zamackni 0");

    io::stdin().read_line(&mut intp_2).expect("Failed to read line");

    let intp: u8 = match intp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let intp_2: u8 = match intp_2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let intp_fin = intp + intp_2;

    let main = rand::thread_rng().gen_range(0..=1);
    let sluzba_1 = rand::thread_rng().gen_range(0..13);
    let sluzba_2 = rand::thread_rng().gen_range(0..14);

    println!("Hlava otrocke operace 1.C je Quatarsky sheik Adam Tybl");
    match intp_fin {
        0 => println!("{}", pr_skupina[sluzba_1]),
        1 => println!("{} jako {}", pr_skupina[sluzba_1], halva[main]),
        3 => println!("{} ", dr_skupina[sluzba_2]),
        4 => println!("{} jako {}", pr_skupina[sluzba_2], halva[main]),
        12 => println!("{} jakozto {}
a  {}", pr_skupina[sluzba_1], halva[main],  dr_skupina[sluzba_2]),
        _ => println!("Ty jsi oplili obcane"),
    }
}
