
use simplelog::*;
use std::fs::*;

// info! for logging
// error! for logging and stdout

pub fn init_logger(){
    let mut options = OpenOptions::new();
    options.write(true).append(true);
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), options.open("logfile.txt").unwrap()),
        ]
    ).unwrap();
}
