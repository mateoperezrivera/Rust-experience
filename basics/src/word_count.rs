use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME:&str="src/resources/word_count.txt";
fn main(){
    let file = File::open(FILENAME).unwrap(); // Open the file
    let reader = BufReader::new(file);
    let mut word_map:HashMap<String,u8>=HashMap::new();
    for line in reader.lines(){
        let unrwapped_line=line.unwrap();
        for word in unrwapped_line.split(" "){
            let undercase=word.to_lowercase();
            match word_map.entry(undercase){
                Entry::Occupied(mut entry) => {
                    entry.insert(entry.get()+1);
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
    }
    let mut pairs:Vec<_>=word_map.drain().collect();
    pairs.sort_by(|a,b|b.1.cmp(&a.1));
    for (key, value) in pairs {
        println!("{} => {}", key, value);
    }
}