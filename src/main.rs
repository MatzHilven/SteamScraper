use std::fs::create_dir_all;
use std::thread::sleep;
use std::time::Duration;

use config::Config;

use steam_scraper::Settings;

mod scraper;

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

    let settings = Settings {
        users: config.get("users").unwrap(),
        scraper_interval: config.get("scraper_interval").unwrap(),
        user_interval: config.get("user_interval").unwrap(),
    };

    if !std::path::Path::new("logs").exists() {
        println!("Creating logs directory");
        create_dir_all("logs").expect("Unable to create logs directory");
    }

    let interval = Duration::from_secs(settings.scraper_interval);
    loop {
        scraper::scrape(settings.clone());
        sleep(interval);
    }
}
