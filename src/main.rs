use std::io;

enum Mood {
    Happy,
    Sad,
    Angry,
    Excited,
    Tired,
    Unknown,
}

fn main() {
    println!("Welcome to mood checker");
    println!("Pls Select an emoji by typing its NUMBER:");
    println!("1. 😊");
    println!("2. 😢");
    println!("3. 😡");
    println!("4. 😃");
    println!("5. 😴");

    
    let mut input = String::new();

   
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choice: u32 = input.trim().parse().unwrap_or(0);

    let mood = match choice {
        1 => Mood::Happy,
        2 => Mood::Sad,
        3 => Mood::Angry,
        4 => Mood::Excited,
        5 => Mood::Tired,
        _ => Mood::Unknown,
    };

    
    match mood {
        Mood::Happy => println!("You are feeling Happy 😊 "),
        Mood::Sad => println!("You are feeling Sad 😢"),
        Mood::Angry => println!("You are feeling Angry 😡"),
        Mood::Excited => println!("You are feeling Excited 😃"),
        Mood::Tired => println!("You are feeling Tired 😴"),
        Mood::Unknown => println!("Unknown selection. Please select a valid emoji."),
    }
}
