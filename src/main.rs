#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr= String::new();
    let mut difficulty = String:: new();
    let mut choice = String::new();

    println!("Input a miner address:");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr).expect("Failed to read line");
    println!("Input a difficulty:");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty).expect("Failed to get in difficulty");
    let diff = difficulty.trim().parse::<u32>().expect("Integer required");
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop{
        println!("Menu");
        println!("1. New Transaction");
        println!("2. Mine block");
        println!("3. Change Difficulty");
        println!("4. Change reward");
        println!("0. Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        println!("");

        match choice.trim().parse().unwrap(){
            0 => {
                println!("Exiting...");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender).expect("Failed to read line");
                print!("Enter receiver address: ");
                io::stdout().flush();  
                io::stdin().read_line(&mut receiver).expect("Failed to read line");
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount).expect("Failed to read line");

                let res = chain.new_transaction(sender.trim().to_string(), 
                receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res{
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            },
            2 => {
                println!("Generating Block");
                let res = chain.generate_new_block();
                match res{
                    true => println!("Block generated"),
                    false => println!("Failed to generate block"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff).expect("Failed to read line");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res{
                    true => println!("Difficulty updated"),
                    false => println!("Failed to update difficulty"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res{
                    true => println!("Reward updated"),
                    false => println!("Failed to update reward"),
                }
            },
            _ => println!("Invalid option"),
        }

    }
}
