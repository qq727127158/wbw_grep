use std::error::Error;
use std::{env, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>,
    ) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        //首参程序名过滤
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    //使用 ? 和特征对象来返回错误
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query){
    //         result.push(line);
    //     }
    // }
    // result

    //函数式写法
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut result = Vec::new();
    //
    // for line in contetns.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    //
    // result

    //函数式写法
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

}

//增加测试模块
#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn one_result() {
        let query = "safe,fast,production";
        let contents = "\
Rust:
safe,fast,production.
Pick three.";

        assert_eq!(vec!["safe,fast,production."],search(query,contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}