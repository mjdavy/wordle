use std::{ fs::File, io::{BufReader, BufRead, Write}};
use text_colorizer::*;
use rand::Rng;

fn main() {
    // print the current working directory
    let cwd = std::env::current_dir().unwrap();
    println!("Current working directory: {}", cwd.display());

    let word_list = load_word_list("words_alpha.txt");

    // extract all five letter words from word_list
    let mut five_letter_words = Vec::new();
    for word in word_list {
        if word.len() == 5 {
            five_letter_words.push(word);
        }
    }

    // pick a random five-letter word
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..five_letter_words.len());
    let random_word = &five_letter_words[random_index];

    // prompt the user to enter a five letter word as a guess at random_word
    // keep prompting until the user enters a five letter word or until six tries
    let mut tries = 0;
    
    
    while tries < 6 {
        let mut guess = String::new();
        let mut char_meta = vec![0; 5]; // 0 = normal, 1 = green bold, 2 = red
        print!("Enter a five letter word: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess.len() != 5 {
            println!("You must enter a five letter word");
            continue;
        }
        if guess == *random_word {
            println!("You guessed the word!");
            break;
        }
        else {
            // compare guess to random_word and print the guess with the correct letters in the correct location green bold
            // print the letters in red if they are in the word but in the wrong position
            // print the letters normally if they are not in the word
            for (i, c) in guess.chars().enumerate() {
                if random_word.contains(c) {
                    if random_word.chars().nth(i).unwrap() == c {
                        char_meta[i] = 1;
                    }
                    else {
                        char_meta[i] = 2;
                    }
                }
            }
            
            print_colorizeed(&guess, &char_meta);
            println!();
            tries += 1;
        }
        guess.clear();
    }
    // print the word in green
    println!("{}", random_word.green().bold());

}

fn load_word_list(words:&str) -> Vec<String>
{
    let mut word_list = Vec::new();
    let file = File::open(words).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        word_list.push(line.unwrap());
    }
    word_list
}

//
// The length of input and char_meta must be the same. This must be validated and give an error if not.
// print the input string according to values in char_meta
// if the value of char_meta at a given index is 0, print the coresponding letter normally
// if the value of char_meta at a given index is 1 print the coresponding character as green bold
// if the value of char_meta at a given index is 2 print the coresponding character as red
//  
fn print_colorizeed(input:&str, char_meta:&Vec<u16>)
{
    for (i, c) in input.chars().enumerate() {
        match char_meta[i] {
            0 => print!("{}", c),
            1 => print!("{}", c.to_string().green().bold()),
            2 => print!("{}", c.to_string().red()),
            _ => print!("{}", c),
        }
    }
}



