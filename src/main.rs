use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::fmt;
use chrono::prelude::*;

#[derive(Debug)]
pub enum ArgType {
    Meeting,
    Exercise,
    Lab,
    Tenta,
    Nada
}

impl fmt::Display for ArgType {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let out;
        match self {
            ArgType::Meeting  => out = "meeting",
            ArgType::Exercise => out = "exercise",
            ArgType::Lab      => out = "lab",
            ArgType::Tenta    => out = "tenta",
            ArgType::Nada     => out = "other"
        }
        write!(f, "{}", out)
    }
}

fn main() {
    let usage =     "Usage: tatt [OPTION] VALUE\n\
                    \n\
                    Options:\n\
                        \t-m : add time for meetings\n\
                        \t-e : add time for exercise sessions\n\
                        \t-l : add time for labs\n\
                        \t-t : add time for exams correction\n\n\
                    Values:\n\
                        \t[+/-] NUMBER [h/m]";
    
    let args: Vec<String> = env::args().collect();

    let mut arg_type : ArgType = ArgType::Nada;
    
    for arg in &args {
        match (&arg_type, arg.as_str()) {
            (ArgType::Nada, "tatt") => (),
            (ArgType::Nada,   "-h") => println!("tatt, TA time tables\n{}", usage),
            (ArgType::Nada,   "-m") => arg_type = ArgType::Meeting,
            (ArgType::Nada,   "-e") => arg_type = ArgType::Exercise,
            (ArgType::Nada,   "-l") => arg_type = ArgType::Lab,
            (ArgType::Nada,   "-t") => arg_type = ArgType::Tenta,
            (ArgType::Nada,   "-g") => generate(),
            (ArgType::Nada,      a) => println!("unknown argument: {}", a),
            (ArgType::Meeting,   s) => add_time(ArgType::Meeting, parse_arg(s)),
            (ArgType::Exercise,  s) => add_time(ArgType::Exercise, parse_arg(s)),
            (ArgType::Lab,       s) => add_time(ArgType::Lab, parse_arg(s)),
            (ArgType::Tenta,     s) => add_time(ArgType::Tenta, parse_arg(s))
        }
    }
}

fn parse_arg(raw : &str) -> f32 {
    return raw.parse::<f32>().unwrap();
}

fn add_time(arg_type : ArgType, num : f32) {
    let month = Local::now().month();
    let day   = Local::now().day(); 

    let file  = format!("/home/nicbot/.tatt/introprogg_{}.txt", month);
    let data  = format!("{}/{} {}: {}\n", day, month, arg_type, num);

    let mut open_file = OpenOptions::new().append(true).open(file).expect("no open");
    open_file.write_all(data.as_bytes()).expect("no write");
}

fn generate() {
    println!("yeet")
}
