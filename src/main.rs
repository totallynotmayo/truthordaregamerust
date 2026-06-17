use std::io::{self, Write};
use rand::prelude::*;


    fn main (){
        let mut exit = 0;
        let mut count = 0;
        let mut rng = rand::rng(); 
            while exit != 1{
                println!("Truth or Dare? (Type random to pick randomly)");
                io::stdout().flush().unwrap();
                let mut input = String::new();
        
                io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
            if input.trim_end() == "dare"{
                let roll = rng.random_range(1..=6); 
                println!("message: {}", roll);
                if roll == 1{
                    println!("Jump 2763 times in public :3");
                }
                else if roll == 2 {
                    println!("Dance in the middle of night no matter where you are >:D");
                }
                else if roll == 3 {
                    println!("Annoy your friend till he burst out >:3");
                }
                else if roll == 4 {
                    println!("Scream as loud as you could X3");
                }
                else if roll == 5 {
                    println!("Add only ONE ice cube in your friend's cup");
                }
                else if roll == 6 {
                    println!("Let yo gng through your phone (O_O)");
                }
                count += 1;
                println!("count: {}", count);
            }
            else if input.trim_end() == "truth"{
                let roll = rng.random_range(1..=6); 
                println!("message: {}", roll);
                if roll == 1{
                    println!("What's the most embarrasing moment in public L_L");
                }
                else if roll == 2 {
                    println!("What's your fav show and what fandom are you in :3");
                }
                else if roll == 3 {
                    println!("Who's your 6th grade crush (˵¬ᴗ¬˵)");
                }
                else if roll == 4 {
                    println!("Have you ever cheated in exam? And what subject was it O_O");
                }
                else if roll == 5 {
                    println!("What is a secret you've kept from your best friend? O_O");
                }
                else if roll == 6 {
                    println!("What is a cringy phase or hyperfixation you used to have? X3");
                }
                count += 1;
                println!("count: {}", count);
            }
            else if input.trim_end() == "exit"{
                println!("Exiting now...");
                println!("total count: {}", count);
                exit = 1;
            }
        }
    }
