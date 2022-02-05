use crate::console::{Choice, input_choice};

mod console;

fn show_new_game_menu() {

}

fn show_load_game_menu() {

}

fn show_options_menu() {

}

fn quit_game() {

}

fn show_main_menu() {
    input_choice(
        "Welcome to Imagination!\nWhat would you like to do?",
        vec![
            Choice::new("New", show_new_game_menu),
            Choice::new("Load", show_load_game_menu),
            Choice::new("Options", show_options_menu),
            Choice::new("Quit", quit_game),
        ]
    )
}

fn main() {
    println!("Imagination is starting...");
    println!(" - Started!");
    show_main_menu();
}
