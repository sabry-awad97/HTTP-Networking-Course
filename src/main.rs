use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
struct DnsResolver {
    client: reqwest::Client,
}

impl DnsResolver {
    fn new() -> DnsResolver {
        DnsResolver {
            client: reqwest::Client::new(),
        }
    }

    async fn resolve(&self, domain: &str) -> Result<Option<String>, Box<dyn Error>> {
        let url = format!(
            "https://cloudflare-dns.com/dns-query?name={}&type=A",
            domain
        );

        let resp = self
            .client
            .get(&url)
            .header("accept", "application/dns-json")
            .send()
            .await?
            .json::<DNSResponse>()
            .await?;
        let answer = resp.answer;
        let ip_address = answer.iter().find_map(|r| {
            if r.record_type == 1 {
                Some(r.data.clone())
            } else {
                None
            }
        });

        Ok(ip_address)
    }
}

#[derive(Debug, Deserialize)]
struct DNSRecord {
    #[serde(rename = "type")]
    record_type: u8,
    data: String,
}

#[derive(Debug, Deserialize)]
struct DNSResponse {
    #[serde(rename = "Answer")]
    answer: Vec<DNSRecord>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let domain = "api.boot.dev";
    let dns_resolver = DnsResolver::new();
    let ip_address = dns_resolver.resolve(domain).await?;

    match ip_address {
        Some(ip) => println!("Found IP address for domain {}: {}", domain, ip),
        None => println!("No IP address found for domain {}", domain),
    }
    Ok(())
}
