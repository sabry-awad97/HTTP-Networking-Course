use std::collections::HashMap;

use crawl::crawl_page;
use report::print_report;

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

    let pages = crawl_page(base_url, base_url, HashMap::new()).await;

    print_report(&pages);
}

// http://wagslane.dev/sitemap.xml
