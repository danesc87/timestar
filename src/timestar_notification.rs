use notify_rust::Notification;
use std::time::Duration;
use std::thread::sleep;
use super::configuration::settings::Settings;

static NOTIFICATION_TITLE: &'static str = "Timestar";
static ICON_NAME: &'static str = "tomato";

pub fn run_timer_and_notify(settings: Settings) {
    let small_break_time = get_duration_from_secs(&settings.time_values.small_break);
    let number_of_timers = settings.number_of_timers;
    for i in 0..number_of_timers {
        notify(&get_timer_number_message(&settings.task_name, i));
        sleep(small_break_time);
        if i + 1 < number_of_timers {
            notify(&settings.notification_messages.small_break);
            sleep(small_break_time);
        }
    }
    notify(&settings.notification_messages.big_break);
    sleep(get_duration_from_secs(&settings.time_values.big_break));
    #[cfg(all(unix, not(target_os = "macos")))]
    // #[cfg(target_os = "macos")]
    notify_with_repetition(settings);
    
    // TODO Check how to repeat all pomodoros again
    #[cfg(target_os = "macos")]
    // #[cfg(all(unix, not(target_os = "macos")))]
    notify(&timer.app_messages.all_tasks_finished);
}

fn get_duration_from_secs(secs: &u64) -> Duration {
    Duration::new(*secs, 0)
}

fn get_timer_number_message(task_name: &String, task_number: u8) -> String {
    let message = format!("Task name: {}\nTask number: {}", task_name, task_number+1);
    message
}

fn notify(message: &str) {
    match Notification::new()
        .summary(NOTIFICATION_TITLE)
        .body(&message)
        .icon(ICON_NAME)
        .appname(NOTIFICATION_TITLE)
        .timeout(2000)
        .show() {
            Ok(_) => (),
            Err(e) => println!("{}", e) 
        }
}

fn notify_with_repetition(settings: Settings) {

    let repeat_message = format!(
        "{}\n{}",
        settings.app_messages.repeat_tasks,
        settings.app_messages.all_tasks_finished
    );

    Notification::new()
        .summary(NOTIFICATION_TITLE)
        .body(&repeat_message)
        .icon(ICON_NAME)
        .appname(NOTIFICATION_TITLE)
        .action("yes", "yes")
        .action("no", "no")
        .timeout(0)
        .show()
        .unwrap()
        .wait_for_action(
            {|action|
                match action {
                    "yes" => run_timer_and_notify(settings),
                    "no" => std::process::exit(1),
                    _ => ()
                }
            }
        )
}
