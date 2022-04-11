/* 
TO-DO: 
    - Implement a control function for BaseGameInfo
    - Add UI elements for the App impl
    NOTE: tutorial video referenced is here - https://www.youtube.com/watch?v=NtUkr_z7l84
*/

#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use eframe::{run_native, NativeOptions, egui::CentralPanel, epi::App, egui::Ui};
// use egui::Ui;

struct BaseGameInfo {
    input_string: String,
    game_string: Vec<String>,
    letter_map: HashMap<char, usize>,
    position_map: Vec<(char, usize)>,
    incorrect_guesses: HashSet<char>
}

// Background stuff
impl BaseGameInfo {
    fn new() -> BaseGameInfo {
        let mut input = String::new();
        BaseGameInfo::get_input(&mut input);
        let maps = BaseGameInfo::give_word_info(&input);
        BaseGameInfo {
            input_string: input,
            game_string: vec![String::from("_"); input.len() - 1],
            letter_map: maps.0.to_owned(),
            position_map: maps.1.to_owned(),
            incorrect_guesses: HashSet::new()
        }
    }

    fn single_loop_step(&mut self, counter: usize) {
        let mut buf: String = String::new();
        let mut guess: char;
        println!("Guess a letter!");
    
        print!("Current word: ");
        for letter in self.game_string.iter() {
            print!("{} ", letter.to_string());
        }
    
        println!();
    
        print!("Incorrect Guesses: ");
        for g in &self.incorrect_guesses {
            print!("{} ", g.to_string());
        }
        
        println!();
    
        BaseGameInfo::get_input(&mut buf);
    
        guess = buf.remove(0);
        
        // Check for membership in the master string thru the map and positions vectors (also gives us access to number of occurances and index)
        if self.letter_map.contains_key(&guess) {
            let num_insertions = *self.letter_map.get(&guess).unwrap() as i32;
    
            for _i in 0..num_insertions {
                let pos = *self.position_map.iter().find(|&&x| x.0 == guess).unwrap();
                let index = self.position_map.iter().position(|&x| x == pos).unwrap();
                self.game_string[pos.1] = String::from(pos.0);
                self.position_map.remove(index);
            }
    
            println!("Good guess!");
            println!();
        } else {
            self.incorrect_guesses.insert(guess);
            counter -= 1;
            println!("Wrong letter! You have {} guesses left!", counter);
            println!();
        }
    
        // println!("{}, {:?}, {:?}, {}", map.contains_key(&guess), self.incorrect_guesses, self.position_map, self.game_string.join("")); --> debug string
    
        // End game checks
        if self.game_string.join("") == self.input_string.to_string() || self.position_map.len() == 0 {
            println!("You got it! With {} guesses left too! The word was {}", counter, self.input_string.to_string());
            // break;
        } else if counter == 10 {
            println!(
                "😬... Sorry, that's not right, and you're all out of guesses. The word was {}",
                self.input_string.to_string()
            );
            // break;
        }
    
        // Empty buffer
        buf = String::new();
    }

    fn get_input(val: &mut String) {
        stdin().read_line(val).ok().expect("Error reading line");
    }

    fn give_word_info(word: &String) -> (HashMap<char, usize>, Vec<(char, usize)>) {
        let mut letter_counts = HashMap::new();
        let mut letter_positions = vec![];

        for (i, letter) in word.chars().enumerate() {
            if letter != '\n' {
                letter_counts.insert(letter.to_owned() as char, word.matches(letter).count());
                letter_positions.push((letter, i))
            }
        }

        return (letter_counts, letter_positions);
    }
}

// GUI stuff
#[allow(unused_variables)]
impl App for BaseGameInfo {
    fn update(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

        });
    }

    fn name(&self) -> &str {
        "Rustman"
    }
}

fn main() {
    let app = BaseGameInfo::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);
}
