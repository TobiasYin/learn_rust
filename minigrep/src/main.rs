use std::env;
use std::error::Error;


fn main() ->Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args)?;
    minigrep::run(config)
}