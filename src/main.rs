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
                    println!("Jump 2763 times in public :3")
                }
                else {
                    println!("Jump over the lazy dog")
                }
                count += 1;
                println!("count: {}", count);
            }
            else if input.trim_end() == "truth"{
                println!("insert truth here :P, message: {}", rng.random_range(1..=6));
                count += 1;
                println!("count {}", count);
            }
            else if input.trim_end() == "exit"{
                println!("Exiting now...");
                println!("total count: {}", count);
                exit = 1;
            }
        }
    }