use std::thread::sleep;
use std::time::Duration;
use steam_scraper::Status;
use crate::Settings;

pub fn scrape(settings: Settings) {
    println!("Scraping users");

    let interval = Duration::from_secs(settings.user_interval);
    for user in settings.users {
        scrape_user(user);
        sleep(interval);
    }
}

fn scrape_user(user: String) {
    println!("Scraping user: {}", user);
    let response = reqwest::blocking::get(format!("{}{}", "https://steamcommunity.com/", user));
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);

    let header_selector = scraper::Selector::parse(".profile_in_game_header").unwrap();

    document.select(&header_selector).for_each(|element| {
        let status: Status = match element.inner_html().as_str() {
            "Currently Offline" => Status::Offline,
            "Currently Online" => Status::Online,
            _ => {
                let game_selector = scraper::Selector::parse(".profile_in_game_name").unwrap();
                let game = document.select(&game_selector).next().unwrap().inner_html();
                Status::Playing(game)
            },
        };
        status.log_with_time(user.as_str());
    });

}
