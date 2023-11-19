extern crate minigrep;
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数の解析に失敗しました
        println!("Problem parsing arguments: {}", err);
        // プログラムを終了します
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // アプリケーションエラー: {}
        println!("Application error: {}", e);
        // プログラムを終了します
        process::exit(1);
    }
    // let mut f = File::open(config.filename).expect("file not found!!");
    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    // // テキストは:\n{}です
    // println!("With text:\n{}", contents);
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1];
//     let filename = &args[2];
//     Config {
//         query: query.clone(),
//         filename: filename.clone(),
//     }
// }
