use std::fs;
use std::fs::*;
use std::fs::{read_to_string, write};
use std::io;
fn main() {
    if !std::path::Path::new("games.txt").exists() {
        File::create("games.txt");
    }
    let path = game_selection();
    let prompt = "\nDu wachst auf. Was tun?\n\n".to_string();
    let mut location = 0;
}

fn io_handler() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(..) => {}
        Err(error) => println!("error: {error}"),
    }
    return input;
}
fn matcher(options: Vec<&str>, prompt: String) -> String {
    println!("{}", prompt);
    for option in options.iter() {
        println!("{}", option);
    }
    let input = &io_handler();
    return *input;
}

fn option_handler(location: &u32) -> Vec<&'static str> {
    let out = vec!["1Hello!", "1Can you hear me?"];
    return out;
}

fn game_selection() -> String {
    let lines = read_to_string("games.txt").expect("Konnte die 'games.txt' Datei nicht finden.");
    let mut open_games: Vec<&str> = lines.split("\n").collect();
    open_games.push("neues Spiel");
    let input: u32 = int(
        open_games,
        "Such dir ein Spiel aus oder fang ein neues an".to_string(),
    );
    if input == (open_games.len()).try_into().unwrap() {
        println!("Gib den Pfad zur Spiel Datei ein:");
        let path = io_handler();
        write("games.txt", path);
        return path.to_string();
    }
    return match open_games.split("\n")[input] {
        Ok(n) => n,
        Err(e) => matcher(
            open_games,
            "Die eingabe war nicht unter den Optionen! Versuchs nochmal.".to_string(),
        ),
    };
}
fn int(options: Vec<&str>, prompt: String) -> u32 {
    return match matcher(options, prompt).parse::<u32>() {
        Ok(n) => n,
        Err(e) => int(
            options,
            format!("Keine nutzbare Zahl eingegeben! {}", prompt),
        ),
    };
}
