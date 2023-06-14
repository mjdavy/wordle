use std::{ fs::File, io::{BufReader, BufRead, Write}, collections::HashSet};
use text_colorizer::*;
use rand::Rng;

fn main() {

    let words = load_word_list("words.txt", 5);
    println!("There are {} five letter words in the word list", words.len());

    // pick a random five-letter word
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..words.len());
    let random_word = words.iter().nth(random_index).unwrap();

    // prompt the user to enter a five letter word as a guess at random_word
    // keep prompting until the user enters a five letter word or until six tries
    let mut tries = 0;

    println!("\n\nWelcome to wordle! Can you guess the word in six tries?\n");
    println!("The word is five letters long. If you guess a letter that is in the word, but not in the correct position, it will be printed in red."); 
    println!("If you guess a letter that is in the word and in the correct position, it will be printed in green bold. If you guess a letter that is not in the word, it will be printed normally.\n");
    
    while tries < 6 {
        let mut guess = String::new();
        let mut char_meta = vec![0; 5]; // 0 = normal, 1 = green bold, 2 = red
       
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();

        // validate the guess - the word must be five letters and in the set of words
        if guess.len() != 5 {
            println!("You must enter a five letter word");
            continue;
        }
        else if !words.contains(&guess) {
            println!("{} is not a word in my list", guess);
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

// load the word list from the file and return a set of all n-letter words where n is between 1 and 10
fn load_word_list(filename:&str, word_length:u8) -> HashSet<String>
{
    if word_length < 1 || word_length > 10 {
        panic!("word length must be between 1 and 10");
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut words_set = HashSet::new();
    for line in reader.lines() {
        let word = line.unwrap();
        if word.len() == word_length as usize {
            words_set.insert(word);
        }
    }
    words_set
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



