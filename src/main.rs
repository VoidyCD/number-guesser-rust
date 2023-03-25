use std::io;
use rand::Rng;


fn main() {

    fn numberGuesser() {

        println!("Guess a number from 1-100");

        
        let mut num = rand::thread_rng().gen_range(1..100);
        let mut input = String::new();
        println!("{}", num);
    
        io::stdin().read_line(&mut input).expect("Give a number.");
    
    
        let x: i32 = input.trim().parse().expect("Input not an integer");
    
        if x == num {
            println!("You guessed The number correctly!")
        }
        else {
            println!("You guessed it wrong.")
        }

        println!("Do you want to guess again? y/n");
        io::stdin().read_line(&mut input).expect("use y/n only");
        
       

    }

    numberGuesser();

    

}
