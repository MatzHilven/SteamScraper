use std::fmt::{Debug, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Settings {
    pub users: Vec<String>,
    pub scraper_interval: u64,
    pub user_interval: u64,
}

pub enum Status {
    Online,
    Offline,
    Playing(String),
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Online => write!(f, "Online"),
            Status::Offline => write!(f, "Offline"),
            Status::Playing(game) => write!(f, "Playing {}", game),
        }
    }
}

impl Status {
    pub fn log_with_time(self, user: &str) {
        let path = format!("logs/{}.log", user.replace("id/", "").replace("profiles/", ""));
        let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let format = format!("{} - {}\n", time, self);

        let mut file = match OpenOptions::new().write(true).append(true).open(path.as_str()) {
            Ok(file) => file,
            Err(_) => File::create(path.as_str()).expect("Unable to create file"),
        };

        file.write_all(format.as_bytes()).expect("Unable to write data");
    }
}
