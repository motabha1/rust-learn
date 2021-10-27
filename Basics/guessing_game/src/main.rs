use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    loop {
        // thread_rng() is the random number generator, while gen_range provides range
        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("Please input your choice: ");

        // We are initiating a new empty object of string
        let mut guess = String :: new();

        // read_line method will return io::Result(enum) which is either Ok (will contain the value) or Err
        // expect is for error handling, will give a warning if not written
        io::stdin().read_line(&mut guess)
            .expect("Problem with line input");

        // we are converting string to u32 by parsing, trim will remove \r\n from the input string    
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; 

        // Like read_line, Ordering will return an enum too
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is smaller than the secret number!"),
            Ordering::Greater => println!("Guess is larger than the secret number"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed {}", guess);

        println!("The secret number is {}", secret_number);
    }

    

}
