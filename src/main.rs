use std::{error::Error, fs::File, io::BufReader, path::Path};
use std::io::{stdin, stdout, Write};
mod domain;
use crate::domain::lang::DB;

fn load_json<P: AsRef<Path>>(path: P) -> Result<DB, Box<dyn Error>> {
    let file = File::open(path)?; 
    let reader = BufReader::new(file); 
    let db = serde_json::from_reader(reader)?; 
    Ok(db)
}

fn input() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    // jsonファイルからプログラミング言語情報を取り出す
    const FILEPATH: &str= "./src/language.json";

    // 問題番号を決める
    let qnum = 1;

    // 問題を出力
    match load_json(FILEPATH) {
        Ok(db) => {
            println!("name: {}", db.language_list[qnum].name);
            for q in db.language_list[qnum].question.iter() {
                println!("question: {}", q);
            }
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    // 標準入力を受け取る
    print!(">> ");
    stdout().flush().unwrap(); 
    let ans = input();

    // 最終結果を出力
    println!("Your answer: {}", ans);
}
