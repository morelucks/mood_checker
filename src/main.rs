fn main (){
    let s1:String=String::from("hello");
    let s2:String=s1.clone();
    println!("{}, wolrd", s1);
}


// use std::io;
// enum Mood {
//     Happy,
//     Sad,
//     Angry,
//     Excited,
//     Tired,
//     Unknown,
// }

// fn main() {
//     println!("Welcome to mood checker");
//     println!("Pls Select an emoji by typing its NUMBER:");
//     println!("1. 😊");
//     println!("2. 😢");
//     println!("3. 😡");
//     println!("4. 😃");
//     println!("5. 😴");

    
//     let mut input = String::new();

   
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     let choice: u32 = input.trim().parse().unwrap_or(0);

//     let mood = match choice {
//         1 => Mood::Happy,    
//         2 => Mood::Sad,
//         3 => Mood::Angry,
//         4 => Mood::Excited,
//         5 => Mood::Tired,
//         _ => Mood::Unknown,
//     };

    
//     match mood {
//         Mood::Happy => println!("You are feeling Happy 😊 "),
//         Mood::Sad => println!("You are feeling Sad 😢"),
//         Mood::Angry => println!("You are feeling Angry 😡"),
//         Mood::Excited => println!("You are feeling Excited 😃"),
//         Mood::Tired => println!("You are feeling Tired 😴"),
//         Mood::Unknown => println!("Unknown selection. Please select a valid emoji."),
//     }
// }
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let mut water_level = 0; // Initial water level
//     let tap_on = true; // Initial state of the tap
//     let tap_off = false; // Initial state of the tap
//     let mut tap_state = tap_off; // Initial state of the tap (off)

//     loop {
//         if water_level <= 15 {
//             tap_state = tap_on;
//             println!("The water level is {}L. The tap is now ON.", water_level);
//         } else if water_level >= 100 {
//             tap_state = tap_off;
//             println!("The water level is {}L. The tap is now OFF.", water_level);
//         } else {
//             println!("The water level is {}L. The tap remains {}.", water_level, if tap_state { "ON" } else { "OFF" });
//         }

//         // Simulate water level changing over time
//         if tap_state == tap_on {
//             water_level += 10; // Increase water level
//         } else {
//             water_level -= 5; // Decrease water level
//         }

//         // Limit water level to reasonable range
//         if water_level > 110 {
//             water_level = 110;
//         } else if water_level < 0 {
//             water_level = 0;
//         }

//         // Pause for a short duration to simulate time passing
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }
