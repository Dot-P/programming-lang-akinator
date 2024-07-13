use std::{error::Error, fs::File, io::BufReader, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Language{
    name: String,
    question: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DB {
    language_list: Vec<Language>
}

fn load_json<P: AsRef<Path>>(path: P) -> Result<DB, Box<dyn Error>> {
    let file = File::open(path)?; 
    let reader = BufReader::new(file); 
    let db = serde_json::from_reader(reader)?; 
    Ok(db)
}

fn main() {
    // jsonファイルからプログラミング言語情報を取り出す
    const FILEPATH: &str= "./src/language.json";

    match load_json(FILEPATH) {
        Ok(db) => {
            println!("name: {}", db.language_list[1].name);
            for q in db.language_list[1].question.iter() {
                println!("question: {}", q);
            }
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
