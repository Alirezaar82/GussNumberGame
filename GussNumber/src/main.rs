use rand::Rng;
use std::io;
fn main() {
    let mut gess = 5;
    let random_number = generate_random_number();

    while gess > 0 {
        let mut user_gess = String::new();
        println!("Enter Your Gess");
        io::stdin().read_line(&mut user_gess).unwrap();

        let user_gess: i32 = match user_gess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                return;
            }
        };

        if random_number == user_gess{
            println!("Congratulations, You Guessed Correctly!");
            println!("The Correct Number was: {}", random_number);
            break;
        }else if random_number < user_gess{
            println!("Too High! Try Again.");
            gess -= 1;
        }else if random_number > user_gess{
            println!("Too Low! Try Again.");
            gess -= 1;
        };
    };

    if gess == 0{
        println!("Sorry, You Lose!");
        println!("The Correct Number was: {}", random_number);
    };

}

fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..101)
}
