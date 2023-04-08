use reqwest::Client;
use std::collections::HashMap;
use url::Url;

use select::document::Document;
use select::predicate::Name;

pub struct Crawler {
    base_url: String,
    visited_pages: HashMap<String, usize>,
}

impl Crawler {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
            visited_pages: HashMap::new(),
        }
    }

    pub async fn crawl(
        &mut self,
        start_url: &str,
        max_depth: usize,
    ) -> Result<&mut HashMap<String, usize>, String> {
        let mut urls_to_visit = vec![(start_url.to_owned(), 0)];
        let client = Client::new();
        let base_url_obj = match Url::parse(&self.base_url) {
            Ok(url) => url,
            Err(err) => return Err(format!("Error parsing base URL: {}", err)),
        };
        let start_resp = match client.get(start_url).send().await {
            Ok(resp) => resp,
            Err(err) => {
                return Err(format!(
                    "Error sending HTTP request to {}: {}",
                    start_url, err
                ))
            }
        };
        if start_resp.status().is_client_error() || start_resp.status().is_server_error() {
            return Err(format!(
                "Error: {} returned status code {}",
                start_url,
                start_resp.status()
            ));
        }

        while let Some((current_url, depth)) = urls_to_visit.pop() {
            if depth > max_depth {
                continue;
            }

            let current_url_obj = match Url::parse(&current_url) {
                Ok(url) => url,
                Err(err) => {
                    println!("Error parsing URL {}: {}", current_url, err);
                    continue;
                }
            };
            if current_url_obj.host_str() != base_url_obj.host_str() {
                continue;
            }
            let normalized_url = match self.normalize_url(&current_url) {
                Some(url) => url,
                None => {
                    println!("Error normalizing URL {}", current_url);
                    continue;
                }
            };
            if let Some(count) = self.visited_pages.get_mut(&normalized_url) {
                *count += 1;
                continue;
            }
            self.visited_pages.insert(normalized_url.clone(), 1);
            println!("Crawling: {}", current_url);
            let resp = match client.get(&current_url).send().await {
                Ok(resp) => resp,
                Err(err) => {
                    println!("Error sending HTTP request to {}: {}", current_url, err);
                    continue;
                }
            };
            let content_type = match resp.headers().get("content-type") {
                Some(value) => match value.to_str() {
                    Ok(s) => s,
                    Err(err) => {
                        return Err(format!(
                            "Error converting content-type header to string: {}",
                            err
                        ))
                    }
                },
                None => {
                    println!("No content-type header in response from {}", current_url);
                    continue;
                }
            };
            if !content_type.contains("text/html") {
                println!(
                    "Got non-html response from {}: {}",
                    current_url, content_type
                );
                continue;
            }

            let html_body = match resp.text().await {
                Ok(body) => body,
                Err(err) => {
                    println!("Error reading response body from {}: {}", current_url, err);
                    continue;
                }
            };
            let next_urls = self.get_urls_from_html(&html_body, &self.base_url);
            for next_url in next_urls {
                if !self.visited_pages.contains_key(&next_url) {
                    urls_to_visit.push((next_url, depth + 1));
                }
            }
        }
        Ok(&mut self.visited_pages)
    }

    fn normalize_url(&self, url: &str) -> Option<String> {
        let parsed_url = match Url::parse(url) {
            Ok(url) => url,
            Err(err) => {
                println!("Error parsing URL {}: {}", url, err);
                return None;
            }
        };
        let host = match parsed_url.host_str() {
            Some(host) => host.to_lowercase(),
            None => {
                println!("No host in URL {}", url);
                return None;
            }
        };
        let path = parsed_url.path().trim_end_matches('/').to_lowercase();
        Some(format!("{}{}", host, path))
    }

    fn get_urls_from_html(&self, html: &str, base_url: &str) -> Vec<String> {
        let document = Document::from(html);
        let mut urls = Vec::new();

        for link in document.find(Name("a")).filter_map(|n| n.attr("href")) {
            match Url::parse(link) {
                Ok(url) => urls.push(url.to_string()),
                Err(_) => {
                    if let Ok(base) = Url::parse(base_url) {
                        if let Ok(resolved) = base.join(link) {
                            urls.push(resolved.to_string());
                        }
                    }
                }
            }
        }

        urls
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    #[test]
    fn test_normalize_url_protocol() {
        let crawler = Crawler::new("https://example.com");
        let input = "https://blog.boot.dev/path";
        let actual = crawler.normalize_url(input).unwrap();
        let expected = "blog.boot.dev/path";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_normalize_url_slash() {
        let crawler = Crawler::new("https://example.com");
        let input = "https://blog.boot.dev/path/";
        let actual = crawler.normalize_url(input).unwrap();
        let expected = "blog.boot.dev/path";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_normalize_url_capitals() {
        let crawler = Crawler::new("https://example.com");
        let input = "https://BLOG.boot.dev/path";
        let actual = crawler.normalize_url(input).unwrap();
        let expected = "blog.boot.dev/path";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_normalize_url_http() {
        let crawler = Crawler::new("https://example.com");
        let input = "http://BLOG.boot.dev/path";
        let actual = crawler.normalize_url(input).unwrap();
        let expected = "blog.boot.dev/path";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_urls_from_html_absolute() {
        let crawler = Crawler::new("https://example.com");
        let input_url = "https://blog.boot.dev";
        let input_body =
            "<html><body><a href=\"https://blog.boot.dev\"><span>Boot.dev></span></a></body></html>";
        let actual = crawler.get_urls_from_html(input_body, input_url);
        let expected = vec!["https://blog.boot.dev/"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_urls_from_html_relative() {
        let crawler = Crawler::new("https://example.com");
        let input_url = "https://blog.boot.dev";
        let input_body =
            "<html><body><a href=\"/path/one\"><span>Boot.dev></span></a></body></html>";
        let actual = crawler.get_urls_from_html(input_body, input_url);
        let expected = vec!["https://blog.boot.dev/path/one"];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_urls_from_html_both() {
        let crawler = Crawler::new("https://example.com");
        let input_url = "https://blog.boot.dev";
        let input_body = "<html><body><a href=\"/path/one\"><span>Boot.dev></span></a><a href=\"https://other.com/path/one\"><span>Boot.dev></span></a></body></html>";
        let actual = crawler.get_urls_from_html(input_body, input_url);
        let expected = vec![
            "https://blog.boot.dev/path/one",
            "https://other.com/path/one",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn test_get_urls_from_html_handle_error() {
        let crawler = Crawler::new("https://example.com");
        let input_url = "https://blog.boot.dev";
        let input_body =
            r#"<html><body><a href="path/one"><span>Boot.dev></span></a></body></html>"#;
        let actual = crawler.get_urls_from_html(input_body, input_url);
        let expected: Vec<String> = vec![];
        assert_eq!(actual, expected);
    }
}
