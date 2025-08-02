use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("{}", content);
    Ok(())
}
//config is created to make connection between find and filename
pub struct Config {
    pub find: String,
    pub filename: String,
}

impl Config {
    // fn parse_input1(args: &[String]) -> (&str, &str) {
    //     let find: &str = &args[1];
    //     let filename: &str = &args[2];
    //     (find, filename)
    // }
    //above is converted to below for better performance

    // fn parse_input2(args: &[String]) -> Config {
    //     let find = &args[1];
    //     let filename = &args[2];
    //     Config {
    //         find: find.to_string(),
    //         filename: filename.to_string(),
    //     }
    // }
    //above is converted to below for better performance

    // fn new(args: &[String]) -> Config {
    //     let find: String = args[1].clone();
    //     let filename: String = args[2].clone();
    //     Config { find, filename }
    // }

    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. at least 2 arguments are needed.");
        }
        let find: String = args[1].clone();
        let filename: String = args[2].clone();
        Ok(Config { find, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}
#[cfg(test)]
mod minigrep_tests {
    use super::*;

    #[test]
    fn test1() {
        let query = "hello";
        let contents = "\
        hello this is test
        checking for some content
        in this file
        is exciting";
        assert_eq!(vec!["in this file"], search(query, contents))
    }
}
