use crawl::Crawler;
use report::print_report;
use std::process;

mod crawl;
mod report;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("no website provided");
        return;
    }
    if args.len() > 2 {
        println!("too many arguments provided");
        return;
    }

    let base_url = &args[1];

    println!("starting crawl of: {}...", base_url);

    let mut crawler = Crawler::new(base_url);

    let result = crawler.crawl(base_url).await;

    match result {
        Ok(pages) => print_report(&pages),
        Err(err) => {
            eprintln!("Crawler failed: {:?}", err);
            process::exit(1);
        }
    }
}

// http://wagslane.dev/sitemap.xml
