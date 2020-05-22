use config::{ConfigError, Config, File};

const FILE_NAME: &str = "timestar.yml";

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct AppMessages {
    pub repeat_tasks: String,
    pub all_tasks_finished: String
}

impl Default for AppMessages {
    fn default() -> Self {
        AppMessages {
            repeat_tasks: String::from("Do you want to repeat pomodoros?"),
            all_tasks_finished: String::from("All of your tasks are finished!")
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct NotificationMessages {
    pub small_break: String,
    pub big_break: String
}

impl Default for NotificationMessages {
    fn default() -> Self {
        NotificationMessages {
            small_break: String::from("Task finished\nHave a little break boss"),
            big_break: String::from("Already completed all taks in this pomodoro session")
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct TimeValues {
    pub task: u64,
    pub small_break: u64,
    pub big_break: u64
}

impl Default for TimeValues {
    fn default() -> Self {
        TimeValues {
            task: 900,
            small_break: 300,
            big_break: 900
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Settings {
    pub task_name: String,
    pub number_of_timers: u8,
    pub time_values: TimeValues,
    pub notification_messages: NotificationMessages,
    pub app_messages: AppMessages
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            task_name: String::from("Development"),
            number_of_timers: 3,
            time_values: TimeValues::default(),
            notification_messages: NotificationMessages::default(),
            app_messages: AppMessages::default()
        }
    }
}

impl Settings {

    // TODO Check how to proper log issues and have a customizable log level
    pub fn new() -> Result<Self, ConfigError> {
        match dirs::config_dir() {
            Some(path) => {
                let full_file_path = format!(
                    "{}/{}",
                    path.as_os_str().to_str().unwrap(),
                    FILE_NAME
                );
                match Self::get_settings(full_file_path.as_str()) {
                    Ok(settings) => Ok(settings),
                    Err(_err) => {
                        println!("{}\nDefault config will be used!\n", _err);
                        Ok(Self::default())
                    }
                }
            },
            None => Ok(Self::default())
        }
    }

    fn get_settings(file_path: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();        
        match settings.merge(File::with_name(file_path)) {
            Ok(_) => settings.try_into(),
            Err(_error) => {
                println!("{}\nDefault config will be used!\n", _error);
                Ok(Self::default())
            }
        }
    }
}


#[cfg(test)]
mod settings_test {

    use super::*;

    #[test]
    fn get_settings_default() {
        let expected_app_messages = AppMessages {
            all_tasks_finished: String::from("All of your tasks are finished!"),
            repeat_tasks: String::from("Do you want to repeat pomodoros?")
        };

        let actual = Settings::new().unwrap();
        assert_eq!(actual.app_messages.all_tasks_finished, expected_app_messages.all_tasks_finished);
    }
}
