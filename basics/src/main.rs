use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rand::{ Rng};

static FILENAME:&str="src/resources/words.txt";
fn main() {
    let word=get_word().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1); // Terminate the program on error
    });
    let mut guessed_letters:Vec<char>=Vec::new();
    let mut wrong_letters:Vec<char>=Vec::new();
    let mut finished=false;
    while !finished {
        print!("current word:");
        for letter in word.chars() {
            if guessed_letters.contains(&letter){
            print!("{}",letter)}
            else{
                print!("_");
            }
        }
        println!();
        print!("wrong guesses:");
        wrong_letters.iter().for_each(|c|{
            print!("{}",c);
        });
        println!();
        println!("input a letter");
        let mut v = String::new();
        io::stdin()
            .read_line(&mut v)
            .expect("Error leyendo la linea.");
        let first_char=v.chars().next().unwrap();
        if first_char.is_alphabetic(){
            if word.contains(first_char) {
                if !guessed_letters.contains(&first_char){
                    guessed_letters.push(first_char);
                }
            }else if  !wrong_letters.contains(&first_char) {
                    wrong_letters.push(first_char);
            }
        }
        if guessed_letters.len()==word.len() {
            finished=true;
            println!("you win!");
        }else if wrong_letters.len()==5 {
            finished=true;
            println!("you lost");
        }
    }

}
fn get_word() -> io::Result<String>{
    let path = Path::new(FILENAME);
    let file_result=File::open(&path);
    match file_result {
        Ok(file) => {
            let reader=BufReader::new(file);
            let mut lines=Vec::new();
            for(_line_number,line) in reader.lines().enumerate(){
                if let Ok(val)=line{
                    lines.push(val);
                }
            }
            let rand_line=lines[rand::thread_rng().gen_range(0..lines.len())].clone();
            Ok(rand_line)
        }
        Err(error) => {
            Err(error)
        }
    }
}
