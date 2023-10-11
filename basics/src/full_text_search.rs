use std::collections::{HashMap, HashSet};
use std::fs::{File, read_dir};
use std::{io, thread};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

const STOP_WORDS: [&str; 15] = [
    "la", "el", "las", "los", "a", ".", ",", ":", ";", "!", "?", "\"", "-", "¿", "¡"
];
fn main(){
    let folder_path = "src/resources/corpus"; // Replace with the path to your folder
    let mut paths:HashSet<PathBuf>=HashSet::new();
    for entry in read_dir(folder_path).unwrap(){
        if let Ok(entry) =entry{
            if entry.path().is_file() && entry.path().extension().is_some_and(|extension| extension == "txt"){
                let path=entry.path();
                paths.insert(path);
            }
        }
    }
    let aux=paths.clone();
    let mut handles = vec![];

    let index: Arc<Mutex<HashMap<String, HashSet<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    for path in aux{
        let handle = thread::spawn(move || {
            create_map(&path)
        });
        // Store the thread handle in the vector
        handles.push(handle);
    }
    for handle in handles {
        let mut map =handle.join().expect("Thread join failed");
        let mut locked=index.lock().unwrap();
        for pair in map.drain(){
            locked.entry(pair.0).or_insert(HashSet::new()).insert(pair.1);
        }
    }
    manage_client(index.lock().unwrap(),paths.len());
}

fn manage_client(index : MutexGuard<HashMap<String, HashSet<String>>>, size: usize) {
    'a: while true {
        let mut v = String::new();
        println!("What you want to search for?");
        io::stdin()
            .read_line(&mut v)
            .expect("Error leyendo la linea.");
        if v == "exit" {
            break 'a;
        }
        let mut points_map:HashMap<&str,f64>=HashMap::new();
        for word in v.split_whitespace()
            .filter(|word| !STOP_WORDS.contains(word))
            .map(|word|word.to_lowercase()){
            if let Some(values)=index.get(word.as_str()){
                for value in values {
                    let num=points_map.entry(value).or_insert(0.0);
                    *num=*num+f64::log((size / values.len()+1) as f64, 10.0);
                }
            }
        }
        let mut pairs:Vec<_>=points_map.iter().collect();
        pairs.sort_by(|a, b| b.1.total_cmp(&a.1));
        for (key, value) in pairs {
            println!("{} => {}", key, value);
        }
    }
}

fn create_map(path: &PathBuf) ->  HashMap<String, String> {
    let mut map:HashMap<String,String>=HashMap::new();
    if let Ok(file)=File::open(path){
        for line in BufReader::new(file).lines(){
            if let Ok(line)=line{
                for word in  line
                    .split(|c| c== ' '|| c== '_')
                    .filter(|word| !STOP_WORDS.contains(word))
                    .map(|word| word.to_lowercase()){
                    map.insert(word,String::from(path.file_name().unwrap().to_str().unwrap()));
                }
            }
        }
    }
    map
}
