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

    let db = match load_json(FILEPATH) {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let total_question: usize = db.question_list.len();

    // 答えが決まるまで質問を繰り返す
    let mut qnum = 0;
    for i in 1..total_question {
        println!("================================================================================");
        println!("{}", db.question_list[qnum]);

        // 標準入力を受け取る
        println!("1: はい 2: 多分そう部分的にそう 3: 分からない 4: 多分違うそうでもない 5: いいえ");
        print!(">> ");
        stdout().flush().unwrap(); 
        let ans = input();

        // 答えをもとに言語の候補を絞る(todo)

        // 回答から次の問題番号を決める(todo)
        qnum = i;
    }

    // 最終結果を出力
    println!("Some output");
}
