use anyhow::Result;
use rss::Channel;
use std::io;

async fn fetch_rss_feed(url: &str) -> Result<Channel, anyhow::Error> {
    let res = reqwest::get(url).await?.text().await?;
    let channel = res.parse::<Channel>()?;
    Ok(channel)
}

fn select_rss_feed() -> &'static str {
    let urls = [
        "https://www.agi.it/cronaca/rss",
        "https://www.agi.it/economia/rss",
        "https://www.agi.it/politica/rss",
        "https://www.agi.it/estero/rss",
        "https://www.agi.it/cultura/rss",
        "https://www.agi.it/sport/rss",
        "https://www.agi.it/innovazione/rss",
        "https://www.agi.it/lifestyle/rss",
    ];

    loop {
        println!("Agi rss number");
        println!("0 to cronaca");
        println!("1 to economia");
        println!("2 to politica");
        println!("3 to estero");
        println!("4 to cultura");
        println!("5 to sport");
        println!("6 to innovazione");
        println!("7 to lifestyle");
        print!("Select number rss: ");

        let mut url = String::new();

        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");

        let num: usize = match url.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return urls[num];
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = select_rss_feed();

    let channel = match fetch_rss_feed(url).await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Error fetching RSS feed: {}", e);
            return Err(e);
        }
    };

    println!("Feed Title: {}", channel.title());
    println!("Feed Description: {}", channel.description());
    println!("Feed Link: {}", channel.link());
    println!("\nItems:\n");

    for item in channel.items() {
        println!("Title: {}", item.title().unwrap_or("No title"));
        println!("Link: {}", item.link().unwrap_or("No link"));
        println!(
            "Description: {}",
            item.description().unwrap_or("No description")
        );
        println!(
            "Publish Date: {}",
            item.pub_date().unwrap_or("No publish date")
        );
        println!();
    }

    Ok(())
}
