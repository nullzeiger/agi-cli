// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

use agi_cli::fetch;
use agi_cli::input_output;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let url = input_output::select_rss_feed();

    let channel = match fetch::rss_feed(url).await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Error fetching RSS feed: {}", e);
            return Err(e);
        }
    };

    input_output::print_channel(channel);

    Ok(())
}
