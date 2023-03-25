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
    //method that takes a arg with Iterator<Item = String> trait
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //skips first env arg, because it is the path
        args.next();

        //gets 2nd env arg for Config.query
        //value is extracted if Some, returns early with Err if None
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get query string"),
        };

        //same as above but for file path
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get file path"),
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .enumerate()
        .collect()
}

#[allow(unused_variables)]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = &query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.contains(query))
        .enumerate()
        .collect()
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
            vec![(0, "safe, fast, productive.")],
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
Trust me.
Crust are crusty";

        // note our test now skip first line
        assert_eq!(
            vec![(0, "Trust me."), (1, "Crust are crusty")],
            search_case_insensitive(query, contents)
        )
    }
}
