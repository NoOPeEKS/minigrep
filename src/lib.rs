use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("Not enough arguments!");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        return Ok(Config{query, file_path})
    }
}

pub fn run (config: Config) {
    let file_contents = fs::read_to_string(config.file_path)
                            .expect("Could not open the file!");
    for line in search(&config.query, &file_contents){
        println!("{line}");
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}