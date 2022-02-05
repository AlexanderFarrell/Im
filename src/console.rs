use std::io::{stdout, Write};

pub struct Choice {
    name: &'static str,
    on: fn(),
}

impl Choice {
    pub fn new(name: &'static str, on: fn()) -> Self {
        Choice { name, on }
    }
}

pub fn input() -> String {
    let mut line = String::new();
    let _=stdout().flush();

    //::io::stdin::read_line(&mut line).expect("error: unable to read user input");
    std::io::stdin().read_line(&mut line).expect("Something went wrong, ending program");
    String::from(line.trim())
    // match std::io::stdin().read_line(&mut line) {
    //     Ok(a) => {
    //         if let Some('\n')=line.chars().next_back() {
    //             line.pop();
    //         }
    //         if let Some('\r')=line.chars().next_back() {
    //             line.pop();
    //         }
    //         line
    //     }
    //     Err(e) => {panic!("An error occurred getting user input from this console.\nThe \
    //     program will now terminate.\nERROR: {}", e)}
    // }
}

pub fn input_valid(validator: fn(&String) -> Result<(), String>, prompt: &'static str) {
    loop {
        println!("{}", prompt);
        print!(" > ");
        let i = input();
        match validator(&i) {
            Ok(_) => {return;}
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

pub fn parse_i32(s: &String) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(a) => {Ok(a)}
        Err(_) => {Err(String::from("Please enter a number without a decimal."))}
    }
}

pub fn input_parse<T>(parser: fn(&String) -> Result<T, String>,
                      prompt: &'static str) -> T {
    loop {
        println!("{}", prompt);
        print!(" > ");
        let i = input();
        match parser(&i) {
            Ok(t) => {return t;}
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

pub fn input_choice(prompt: &'static str, choices: Vec<Choice>) {
    let mut next: Option<fn()> = None;

    while next.is_none() {
        println!("\n{}", prompt);
        print!(" > ");
        let choice = input();
        let mut i = 0;
        while i < choices.len() {
            if choices[i].name.to_lowercase() == choice.trim().to_lowercase() {
                next = Some(choices[i].on);
                i = choices.len();
            }
            i += 1;
        }
        if next.is_none() {
            println!("Invalid Input");
        }
    }
    next.unwrap()();
}