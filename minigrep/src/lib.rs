use std::io;
use std::error::Error;

pub struct Config<'a> {
    filename: &'a str,
    query: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("no enough args");
        }
        let search_str = &args[1];
        let filename = &args[2];
        Ok(Config {
            filename,
            query: search_str,
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let res = search(config.filename, config.query)?;
    for i in res {
        println!("{}", i)
    };
    Ok(())
}

fn search(filename: &str, search: &str) -> Result<Vec<String>, io::Error> {
    let content = read_file(filename)?;
    Ok(content.lines().filter(|i| {
        i.contains(search)
    }).map(|i| String::from(i)).collect())
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(filename)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new() {
        let arg = vec!["a", "b"].iter().map(|x| String::from(*x)).collect::<Vec<String>>();
        let cfg1 = Config::new(&arg);
        assert!(cfg1.is_err());
    }
}