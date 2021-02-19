use console::Term;
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};
use std::process;
use std::thread;
use std::time::Duration;

extern crate rand;

// go ahead
// make a pr so they're not global anymore
static mut NUMBER_MEMORY_SCORE: u8 = 0;

fn clear() {
    Term::stdout().clear_screen().expect("it broke");
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time))
}

fn main() {
    println!("Hello, let's get started!\n\nSelect a test:\n1: Number Memory");
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            if input.ends_with('\n') {
                input.pop();
                if input.ends_with('\r') {
                    input.pop();
                }
            }
            clear();
            if input == "1" {
                number_memory();
            }
        }
        Err(e) => println!("how: {}", e),
    }
}

fn dashboard() {
    unsafe {
        println!("Number Memory: {}\n\nEnter to go back"
        ,NUMBER_MEMORY_SCORE);
        pause();
        // for now it quits
        process::exit(1);
    }
}

fn number_memory() {
    println!("The average person remembers 7 numbers, how many can you remember?");
    sleep(5);
    clear();

    let mut lost = false;
    let mut number = String::new();
    let mut counter: u8 = 1;

    while !lost {
        number.push_str(&rand::thread_rng().gen_range(0, 10).to_string());

        println!("{}", number);
        sleep(10);
        clear();

        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                clear();
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }

                if number == input {
                    println!("Correct! Your score is now {}!", counter);
                    sleep(5);
                    clear();
                } else {
                    lost = true;
                    println!(
                        "Wrong! It was '{}' and you answered '{}'\nYour score is {}",
                        number, input, counter
                    );
                    pause();
                    clear();
                }
            }
            Err(e) => println!("how: {}", e),
        }
        counter += 1;
    }
    unsafe {
        NUMBER_MEMORY_SCORE = counter;
    }
}
