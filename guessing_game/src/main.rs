use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    /*
       # 'let' creates a variable. In rust, all variables bydefault are immutable.
       # String is a string type provided by standard lib
       # new() is associated function (or called static function in another lanuages)
       # '&' is used to pass as reference to avoid copy same variable in memory
       # crate is a collection of Rust source code files. This code is also a crate but binary crate not the library.
       # For crates dependencies, during first time build, cargo will download the latest versions even if you have specified lower version in `cargo.toml` file .
        (eg: specified rand =0.8.3 but downloaded version is 0.8.5)
       # Cargo writes all downloaded version(s) in cargo.lock file sothat same versioning can be used for future builds.
       # cargo update => ignores Cargo.lock file and download with latest version compatibile with semantic versioning. (eg: 0.8.9 but not 0.9.0)
       # Incase, we need minor or major version, then update directly in cargo.toml file
       # To generate a document for help: cargo doc --open
    */

    println!("**************************************");
    println!("Lets play Game: Guessing the number");
    println!("**************************************");

    // secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // user input
    loop {
        println!("Please enter the number to be guessed:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Entered number is wrong.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter number should be in numeric format");
                continue;
            }
        };

        println!("Your guessing number is: {}", guess);

        // compare

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Entered number is too small"),
            Ordering::Greater => println!("Entered number is too big"),
            Ordering::Equal => {
                println!("YOU WON !!!");
                break;
            }
        }
    }
}
