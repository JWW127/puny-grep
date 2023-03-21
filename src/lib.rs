use colored::*;
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //env requires at least 2 args
        if args.len() < 3 {
            // our Result<&'static str>
            return Err("\x1b[31mNot Enough Args\x1b[0m");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // our Result<Config>
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// accepts our config instance returns result
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // this opens the file gives us a result whos success
    // is a string of the contents of the file
    #[allow(unused_variables)]
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        let l_num = line.0;
        let l_text = line.1.to_string();

        println!(
            "\"{}\" {} {} \n  {}\n",
            &config.query,
            "line:".bright_cyan().bold(),
            &l_num,
            &l_text.green(),
        );
    }
    Ok(())
}

#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut res = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.contains(query) {
            res.push(line);
        }
    }
    res // this Vec<&str> is returned and looped in run()
}

#[allow(unused_variables)]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines().enumerate() {
        if line.1.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res // this Vec<&str> is returned and looped in run()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query, contents)
        )
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
            vec![(0, "Rust:"), (3, "Trust me.")],
            search_case_insensitive(query, contents)
        )
    }
}
