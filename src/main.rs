use crawl::Crawler;
use report::print_report;
use std::{error::Error, io::stdout, process};

mod crawl;
mod report;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("no website provided");
        process::exit(1);
    }
    if args.len() > 2 {
        println!("too many arguments provided");
        process::exit(1);
    }

    let base_url = &args[1];

    println!("starting crawl of: {}...", base_url);

    let mut crawler = Crawler::new(base_url);

    let pages = crawler.crawl(base_url, 5).await?;
    print_report(&pages, &mut stdout())?;

    Ok(())
}

// http://wagslane.dev/sitemap.xml
