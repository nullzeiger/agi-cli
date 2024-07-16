// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

pub mod fetch {
    pub async fn rss_feed(url: &str) -> Result<rss::Channel, anyhow::Error> {
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        let channel = content.parse::<rss::Channel>()?;
        Ok(channel)
    }
}

pub mod input_output {
    pub fn without_tags(text: String) -> String {
        text.replace("<p>", "")
            .replace("</p>", "")
            .replace("<strong>", "")
            .replace("</strong>", "")
            .replace("<h2>", "")
            .replace("</h2>", "")
            .replace("<br>", "")
            .replace("&nbsp", "")
            .replace(".;", ".")
    }

    pub fn select_rss_feed() -> &'static str {
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

            std::io::stdin()
                .read_line(&mut url)
                .expect("Failed to read line");

            let num: usize = match url.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            return urls[num];
        }
    }

    pub fn print_channel(channel: rss::Channel) {
        println!("Feed Title: {}", channel.title());
        println!("Feed Description: {}", channel.description());
        println!("Feed Link: {}", channel.link());
        println!("\nItems:\n");

        for item in channel.items() {
            println!("Title: {}", item.title().unwrap_or("No title"));
            println!("Link: {}", item.link().unwrap_or("No link"));
            println!(
                "Description: {}",
                without_tags(item.description().unwrap_or("No description").to_string())
            );
            println!(
                "Publish Date: {}",
                item.pub_date().unwrap_or("No publish date")
            );
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_tags() {
        let text = "<p>Hello</p> <strong>World</strong>".to_string();
        assert_eq!(input_output::without_tags(text), "Hello World");
    }

    #[tokio::test]
    async fn test_valid_rss_feed() -> Result<(), anyhow::Error> {
        let url = "https://www.agi.it/cronaca/rss";
        let channel = fetch::rss_feed(url).await?;
        assert!(!channel.title.is_empty());
        Ok(())
    }
}
