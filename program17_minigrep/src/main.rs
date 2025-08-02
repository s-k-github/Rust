use program17_minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing elements : {}", err);
        process::exit(1)
    });
    println!("Finding : {}", config.find);
    println!("In file : {}", config.filename);
    //if run return error rhen execute mentioned block is the meaning of below
    if let Err(e) = program17_minigrep::run(config) {
        println!("Application error : {}", e);
        process::exit(1)
    };
}
