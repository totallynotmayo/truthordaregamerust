use std::io::{self, Write};
use rand::prelude::*;
use std::{thread};
use std::time::Duration;


    fn main (){
        let gamestart = 1;
        let mut count = 0;
        let mut rng = rand::rng(); 
        // made an array instead of making a pile of "if and else" statements
        let dare: [&str; 6]= ["Jump 2763 times in public :3", 
        "Dance in the middle of night no matter where you are >:D",
        "Annoy your friend till he burst out >:3",
        "Scream as loud as you could X3",
        "Add only ONE ice cube in your friend's cup",
        "Let yo gng through your phone (O_O)"];

        let truth: [&str; 6]= ["What's the most embarrasing moment in public L_L",
        "What's your fav show and what fandom are you in :3",
        "Who's your 6th grade crush (˵¬ᴗ¬˵)",
        "Have you ever cheated in exam? And what subject was it O_O",
        "What is a secret you've kept from your best friend? O_O",
        "What is a cringy phase or hyperfixation you used to have? X3"];

            while gamestart != 0{
                println!("Truth or Dare? (Type \"exit\" to quit)");
                io::stdout().flush().unwrap();
                let mut input = String::new();
        
                io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
                if input.trim_end().to_lowercase() == "dare"{
                    let roll = rng.random_range(0..=5); 
                    println!("message: {}", roll);
                    println!("{}", dare[roll]);
                    count += 1;
                    println!("count: {}\n", count);
                    thread::sleep(Duration::from_secs(1));
                }
                else if input.trim_end().to_lowercase() == "truth"{
                    let roll = rng.random_range(0..=5); 
                    println!("message: {}", roll);
                    println!("{}", truth[roll]);
                    count += 1;
                    println!("count: {}\n", count);
                    thread::sleep(Duration::from_secs(1));
                }
                else if input.trim_end().to_lowercase() == "sixseven"{
                    print!("what nonsense are you talking again?\n");
                }
                else if input.trim_end().to_lowercase() == "exit"{
                    println!("Exiting now...");
                    println!("total count: {}", count);
                    thread::sleep(Duration::from_secs(1));
                    break;
            }
        }
    }
