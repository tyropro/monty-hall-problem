// Monty Hall Problem
// https://en.wikipedia.org/wiki/Monty_Hall_problem

// Step 1: Choose a winning door
// Step 2: User chooses a door
// Step 3: Open an incorrect door
// Step 4: Ask for a switch
// Step 5: Reveal the result

// Step 6: Record the result (win or loss)
// Step 7: Calculate the win rate (win / total * 100)
// Step 8: Loop

use rand::Rng; 

fn main() {
    println!("Monty Hall Problem");
    println!("by Kaorai\n");

    let mut total_plays: i32 = 0;
    let mut wins: i32 = 0;

    loop {
        let winning_door: i8 = rand::thread_rng().gen_range(1..4);

        let mut user_choice: i8 = 0;

        while user_choice > 3 || user_choice < 1 {
            println!("Choose a door (1, 2, 3) or type 'exit' to quit: ");

            let mut user_input: String = String::new();
            std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
            user_input = user_input.trim().to_string();

            if user_input == "exit" {
                return;
            }

            user_choice = user_input.parse().expect("Please enter a number");
        }

        let mut incorrect_door: i8 = user_choice;
        if user_choice == winning_door {
            while incorrect_door == winning_door || incorrect_door == user_choice {
                incorrect_door = rand::thread_rng().gen_range(1..4);
            }
        } else {
            incorrect_door = 6 - user_choice - winning_door;
        }
        
        println!("\nDoor {} is incorrect", incorrect_door);
        println!("Would you like to switch doors? (y or n)");

        let mut user_input: String = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input = user_input.trim().to_string();

        if user_input == "y" {
            user_choice = 6 - user_choice - incorrect_door;
        }

        println!("\nUser choice: {}", user_choice);
        println!("Winning door: {}", winning_door);
        if user_choice == winning_door {
            println!("You win!");
            wins += 1;
        } else {
            println!("You lose!");
        }

        total_plays += 1;

        println!("Win rate: {:.2}%\n", (wins as f32 / total_plays as f32) * 100.0);
    }
}
