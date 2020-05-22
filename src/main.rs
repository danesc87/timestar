mod configuration;
mod timestar_notification;

use std::env::args;
use timestar_notification::run_timer_and_notify;
use configuration::settings::Settings;

// Crates for reading config files
extern crate dirs;
extern crate serde;
#[macro_use]
extern crate serde_derive;


fn main() {
    // TODO Allow argunments for special cases
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
         let _argument = &args[1];
    }

    let timestar_settings = Settings::new().unwrap();
    run_timer_and_notify(timestar_settings);
}
