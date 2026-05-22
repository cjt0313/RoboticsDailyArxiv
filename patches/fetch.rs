use super::structs::{Arxiv, ArxivQuery};
use crate::{ArxivCollection, Config};
use anyhow::Result;
use reqwest::{Client, IntoUrl, StatusCode};
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::path::Path;
use tracing::{info, warn};
use xml::reader::{EventReader, XmlEvent};

const PAGE_SIZE: i32 = 50;

pub async fn feed_cache<T, S>(url: T, client: &Client) -> Result<S>
    where
        T: IntoUrl,
        S: DeserializeOwned,
{
    Ok(client.get(url).send().await?.json().await?)
}

pub async fn from_cache(url: &Option<String>, client: &Client) -> ArxivCollection {
    if let Some(cache_url) = url {
        info!("Feeding rss cache from {}", cache_url);
        match feed_cache(cache_url, &client).await {
            Ok(rss) => {
                info!("Feed rss cache Successfully!");
                rss
            }
            Err(err) => {
                warn!("Failed: {}!", err.to_string());
                Default::default()
            }
        }
    } else {
        Default::default()
    }
}

pub fn dump_cache(cache_data: &ArxivCollection, config: &Config) -> Result<()> {
    fs::create_dir_all(&config.target_dir)?;
    let cache_path = Path::new(&config.target_dir).join("cache.json");

    info!("Dumping Cache: {}", cache_path.to_string_lossy());
    let mut f = File::create(cache_path)?;
    serde_json::to_writer(&mut f, &cache_data)?;
    Ok(())
}

pub async fn fetch_arxivs(query: ArxivQuery, client: &Client) -> Result<Vec<Arxiv>> {
    let total = query.max_results.unwrap_or(PAGE_SIZE);
    let mut all_arxivs = Vec::new();
    let mut offset = query.start.unwrap_or(0);

    while (offset - query.start.unwrap_or(0)) < total {
        let batch_size = std::cmp::min(PAGE_SIZE, total - (offset - query.start.unwrap_or(0)));
        let mut page_query = query.clone();
        page_query.start = Some(offset);
        page_query.max_results = Some(batch_size);

        info!("  Fetching papers {}-{}...", offset, offset + batch_size);
        let arxivs = fetch_page(&page_query, client).await?;
        let count = arxivs.len() as i32;
        all_arxivs.extend(arxivs);

        if count < batch_size {
            break;
        }

        offset += batch_size;
        if (offset - query.start.unwrap_or(0)) < total {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
    }

    Ok(all_arxivs)
}

async fn fetch_page(query: &ArxivQuery, client: &Client) -> Result<Vec<Arxiv>> {
    let url = query.to_url();
    let max_retries: u32 = 10;
    for attempt in 0..max_retries {
        let resp = client.get(&url)
            .header("User-Agent", "RoboticsDailyArxiv/1.0 (https://github.com/cjt0313/RoboticsDailyArxiv)")
            .send().await?;
        if resp.status() == StatusCode::OK {
            let body = resp.text().await?;
            if body.contains("<html") || body.contains("Rate exceeded") {
                let delay = 60 + attempt * 30;
                warn!("Rate limited (attempt {}/{}), waiting {}s...", attempt + 1, max_retries, delay);
                tokio::time::sleep(std::time::Duration::from_secs(delay.into())).await;
                continue;
            }
            return parse_data(body);
        } else {
            let delay = 60 + attempt * 30;
            warn!("HTTP {} (attempt {}/{}), waiting {}s...", resp.status(), attempt + 1, max_retries, delay);
            tokio::time::sleep(std::time::Duration::from_secs(delay.into())).await;
        }
    }
    anyhow::bail!("Failed to fetch arxiv data after {} retries", max_retries)
}

fn parse_data(body: String) -> Result<Vec<Arxiv>> {
    let mut parser = EventReader::from_str(&body);
    let mut arxiv = Arxiv::new();
    let mut arxivs = Vec::new();

    'outer: loop {
        match parser.next()? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => match &name.local_name[..] {
                "entry" => {
                    arxiv = Arxiv::new();
                }
                "id" => {
                    if let XmlEvent::Characters(id) = parser.next()? {
                        arxiv.id = id;
                    }
                }
                "updated" => {
                    if let XmlEvent::Characters(updated) = parser.next()? {
                        arxiv.updated = updated.parse()?
                    }
                }
                "published" => {
                    if let XmlEvent::Characters(published) = parser.next()? {
                        arxiv.published = published.parse()?
                    }
                }
                "title" => {
                    if let XmlEvent::Characters(title) = parser.next()? {
                        arxiv.title = title
                    }
                }
                "summary" => {
                    if let XmlEvent::Characters(summary) = parser.next()? {
                        arxiv.summary = summary
                    }
                }
                "author" => {
                    parser.next()?;
                    parser.next()?;
                    if let XmlEvent::Characters(author) = parser.next()? {
                        arxiv.authors.push(author);
                    }
                }
                "link" => {
                    if attributes[0].value == "pdf" {
                        arxiv.pdf_url = format!(
                            "{}.pdf",
                            attributes[1].value.replacen("http", "https", 1).clone()
                        );
                    }
                }
                "comment" => {
                    if let XmlEvent::Characters(comment) = parser.next()? {
                        arxiv.comment = Some(comment);
                    }
                }
                _ => (),
            },
            XmlEvent::EndElement { name } => match &name.local_name[..] {
                "entry" => {
                    arxivs.push(arxiv.clone());
                }
                "feed" => {
                    break 'outer;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(arxivs)
}
