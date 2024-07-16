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
    let total_language: usize = db.language_list.len();

    // 答えが決まるまで質問を繰り返す
    let mut qnum = 0;

    const BETA :f32 = 0.05;
    let mut p_lang = vec![(1/total_language) as f32; total_language]; 
    let mut h = vec![0.0; total_language];

    for i in 1..total_question {
        println!("================================================================================");
        println!("{}", db.question_list[qnum]);

        // 標準入力を受け取る
        println!("1: はい 2: 多分そう部分的にそう 3: 分からない 4: 多分違うそうでもない 5: いいえ");
        print!(">> ");
        stdout().flush().unwrap(); 
        let input: i32 = input();

        // 答えをもとに言語の候補を絞る(todo)
        let alpha :f32  = match input {
            1 => 1.0,
            2 => 0.5,
            3 => 0.0,
            4 => -0.5,
            5 => -1.0,
            _ => {
                println!("Invalid input");
                return;
            }
        };

        let mut a = Vec::new();

        for j in 0..total_language{
            match db.language_list[j].answer[qnum].as_str(){
                "Yes" => a.push(1.0),
                "No" => a.push(-1.0),
                _ => (),
            };
        }

        // H_{i,n}の更新
        for j in 0..total_language{
            h[j] += (alpha - a[j]).powi(2);
        }

        // P_{lang}の更新
        let mut tmp = Vec::new();
        for &value in &h {
            tmp.push((-BETA * value).exp());
        }
        for j in 0..total_language{
            p_lang[j] = tmp[j] / tmp.iter().sum::<f32>();
            if p_lang[j] > 0.8 {
                println!("あなたが思い浮かべているのは {} ですか？", db.language_list[j].name);
                std::process::exit(0);
            }
        }

        // 現在の確率分布を表示
        for j in 0..total_language{
            println!("{}: {}", db.language_list[j].name, p_lang[j]);
        }

        // 回答から次の問題番号を決める(todo)
        qnum = i;
    }

    // 最も確率が高い言語を表示
    let mut max = 0;
    for i in 0..total_language{
        if p_lang[i] > p_lang[max]{
            max = i;
        }
    }
    println!("あなたが思い浮かべているのは {} ですか？", db.language_list[max].name);
}
