mod configuration;
mod timestar_notification;

use timestar_notification::run_timer_and_notify;
use configuration::settings::Settings;

// Crates for reading config files
extern crate dirs;
extern crate serde;
#[macro_use]
extern crate serde_derive;


fn main() {
    let timestar_settings = Settings::new().unwrap();
    run_timer_and_notify(
        timestar_settings,
        parse_timestar_repetition_argument(
            std::env::args().collect()
        )
    );
}


fn parse_timestar_repetition_argument(args: Vec<String>)-> u8 {
    let mut repeat: u8 = 0;
    if args.len() > 1 {
        match args[1].parse::<u8>() {
            Ok(r) => repeat = r,
            Err(_) =>  {
                println!("Argument must be a postivie integer");
                std::process::exit(1);
            }
        }
    };
    repeat
}