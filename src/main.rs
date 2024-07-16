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

fn shanon_entropy(p: Vec<f32>) -> f32 {
    let mut h = 0.0;
    for i in 0..p.len(){
        h += p[i] * p[i].log2();
    }
    -h
}

fn argmax<T: PartialOrd>(slice: &[T]) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }
    let mut max_index = 0;
    for (i, item) in slice.iter().enumerate() {
        if item > &slice[max_index] {
            max_index = i;
        }
    }
    Some(max_index)
}

fn calculate_question_entropy(db: &DB, p_lang: &Vec<f32>, question_index: usize) -> f32 {
    let mut yes_prob = 0.0;
    let mut no_prob = 0.0;

    for (i, lang) in db.language_list.iter().enumerate() {
        if db.language_list[i].answer[question_index] == "Yes" {
            yes_prob += p_lang[i];
        } else if db.language_list[i].answer[question_index] == "No" {
            no_prob += p_lang[i];
        }
    }

    shanon_entropy(vec![yes_prob, no_prob])
}

fn main() {
    // jsonファイルからプログラミング言語情報を取り出す
    const FILEPATH: &str= "./src/language_v1.json";

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

        // 答えをもとに言語の候補を絞る
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
            if p_lang[j] > 1.0 / total_language as f32 * 2.0{
                println!("あなたが思い浮かべているのは {} ですか？", db.language_list[j].name);
                std::process::exit(0);
            }
        }

        // 現在の確率分布を表示
        for j in 0..total_language{
            println!("{}: {}", db.language_list[j].name, p_lang[j]);
        }

        // 次の質問を計算
        let mut question_entropies = vec![0.0; total_question];
        for q in 0..total_question {
            question_entropies[q] = calculate_question_entropy(&db, &p_lang, q);
        }
        qnum = argmax(&question_entropies).unwrap();
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
