use::std::env;
use std::{fs, process};
use wbw_grep::Config;

fn main() {
    //let args:Vec<String> = env::args().collect();
    //dbg!(args);
    //存储读取到的参数
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching word is {}",config.query);
    println!("In file path is {}",config.file_path);
    println!("================================================");

    if let Err(e) =  wbw_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
    //$env:IGNORE_CASE=1; cargo run -- liKe text.txt  win10下设置环境变量命令
}


