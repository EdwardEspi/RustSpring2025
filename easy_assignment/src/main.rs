use std::fs::File;
use std::io::prelude::*;

struct Config {
    your_name: String,
    s_id: String,
    port: u16,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let your_name = lines.next().unwrap().to_string();
        let s_id = lines.next().unwrap().to_string();
        let port = lines.next().unwrap().parse().unwrap();

        Config { your_name, s_id, port }
    }
}

fn main() {
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.your_name);
    println!("SID: {}", config.s_id);
    println!("Port: {}", config.port);
}