use std::error::Error;
use std::{env, fs};

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // 使用链式调用传递错误信息
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);
  
  println!("{}", &config.case_sensitive);

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

// 大小写敏感的查询
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  // 遍历文件内容的每一行，该行包含查询的内容就插入到 Vector （动态数组）中
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

// 大小写不敏感的查询
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  let query = query.to_lowercase();

  // 遍历文件内容的每一行，该行包含查询的内容就插入到 Vector （动态数组）中
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}