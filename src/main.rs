use crate::console::{input, input_choice, input_parse, input_prompt, parse_i32, Choice};
use std::time::Duration;

mod console;

struct Player {
    name: String,
    age: i32,
}

fn start_game(player: Player) {
    println!("Wonderful!");
    sleep(0.5);
    println!("Nice to meet you {}", player.name);
    sleep(0.5);
    println!("You fall into a deep sleep and awake in a far away land.");
}

fn sleep(seconds: f64) {
    let duration = Duration::from_secs_f64(seconds);
    std::thread::sleep(duration);
}

fn show_new_game_menu() {
    let name = input_prompt("What is your name?");
    let age = input_parse(parse_i32, "How old are you? (years)");
    start_game(Player { name, age });
}

fn show_load_game_menu() {}

fn show_options_menu() {}

fn quit_game() {}

fn show_main_menu() {
    input_choice(
        "Welcome to Imagination!\nWhat would you like to do?",
        vec![
            Choice::new("New", show_new_game_menu),
            Choice::new("Load", show_load_game_menu),
            Choice::new("Options", show_options_menu),
            Choice::new("Quit", quit_game),
        ],
    )
}

fn main() {
    println!("Imagination is starting...");
    println!(" - Started!");
    show_main_menu();
}
