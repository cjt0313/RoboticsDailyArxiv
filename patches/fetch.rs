use super::structs::{Arxiv, ArxivQuery};
use crate::{ArxivCollection, Config};
use anyhow::Result;
use chrono;
use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::path::Path;
use tracing::{info, warn};
use xml::reader::{EventReader, XmlEvent};

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
    let category = query.search_query.strip_prefix("cat:").unwrap_or(&query.search_query);
    let rss_url = format!("https://rss.arxiv.org/rss/{}", category);
    info!("  Fetching RSS feed from {}", rss_url);

    let resp = client.get(&rss_url)
        .header("User-Agent", "RoboticsDailyArxiv/1.0 (https://github.com/cjt0313/RoboticsDailyArxiv)")
        .send().await?;
    let body = resp.text().await?;
    parse_rss(body)
}

fn parse_rss(body: String) -> Result<Vec<Arxiv>> {
    let mut parser = EventReader::from_str(&body);
    let mut arxivs = Vec::new();

    let mut in_item = false;
    let mut current_tag = String::new();
    let mut title = String::new();
    let mut link = String::new();
    let mut description = String::new();
    let mut creators = String::new();
    let mut pub_date = String::new();

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let tag = name.local_name.clone();
                if tag == "item" {
                    in_item = true;
                    title.clear();
                    link.clear();
                    description.clear();
                    creators.clear();
                    pub_date.clear();
                }
                if in_item {
                    current_tag = tag;
                }
            }
            Ok(XmlEvent::Characters(text)) | Ok(XmlEvent::CData(text)) => {
                if in_item {
                    match current_tag.as_str() {
                        "title" => title.push_str(&text),
                        "link" => link.push_str(&text),
                        "description" => description.push_str(&text),
                        "creator" => creators.push_str(&text),
                        "pubDate" => pub_date.push_str(&text),
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                let tag = name.local_name.clone();
                if tag == "item" {
                    in_item = false;
                    current_tag.clear();

                    let id = link.clone();
                    let pdf_url = link.replace("/abs/", "/pdf/") + ".pdf";
                    let pdf_url = pdf_url.replacen("http://", "https://", 1);

                    let summary = if let Some(abs_start) = description.find("Abstract:") {
                        description[abs_start + 9..].trim().to_string()
                    } else {
                        description.trim().to_string()
                    };

                    let authors: Vec<String> = creators
                        .split(',')
                        .map(|a| {
                            let a = a.trim();
                            if let Some(paren) = a.find('(') {
                                a[..paren].trim().to_string()
                            } else {
                                a.to_string()
                            }
                        })
                        .filter(|a| !a.is_empty())
                        .collect();

                    let date = if let Ok(d) = chrono::DateTime::parse_from_rfc2822(&pub_date) {
                        d.with_timezone(&chrono::Utc)
                    } else {
                        chrono::Utc::now()
                    };

                    let comment = extract_comment(&description);

                    if !title.is_empty() {
                        arxivs.push(Arxiv {
                            id,
                            updated: date,
                            published: date,
                            title: title.trim().to_string(),
                            summary,
                            authors,
                            pdf_url,
                            comment,
                        });
                    }
                } else if in_item {
                    current_tag.clear();
                }
            }
            Ok(XmlEvent::EndDocument) => break,
            Err(_) => break,
            _ => {}
        }
    }

    info!("  Parsed {} papers from RSS", arxivs.len());
    Ok(arxivs)
}

fn extract_comment(description: &str) -> Option<String> {
    if let Some(idx) = description.find("Comment:") {
        let after = &description[idx + 8..];
        let end = after.find('\n').unwrap_or(after.len());
        let comment = after[..end].trim().to_string();
        if !comment.is_empty() {
            return Some(comment);
        }
    }
    None
}
