use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //env requires at least 2 args
        if args.len() < 3 {
            // our Result<&'static str>
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // our Result<Config>
        Ok(Config { query, file_path })
    }
}

// accepts our config instance returns result
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // this opens the file gives us a result whos success
    // is a string of the contents of the file
    #[allow(unused_variables)]
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res // this Vec<&str> is returned and looped in run()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
