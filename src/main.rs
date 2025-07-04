use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::Url;
use serde_derive::{Serialize, Deserialize};
use serde_bencode::{self, from_bytes};
use serde;
use sha1::{Digest, Sha1};
use std::{any::{self, Any}, fs, string};
use std::collections::HashMap;
use chrono::{NaiveDate, DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::error::Error;
use rand::prelude::*;
// use tokio;


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    announce: Option<String>,
    comment: Option<String>,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date", with = "chrono::serde::ts_seconds_option")]
    creation_date: Option<DateTime<Utc>>,
    #[serde(rename = "created by")]
    created_by: Option<String>,
    info: Info,
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct  Info {
    name: String,
    // #[serde(with = "serde_bytes")]
    // pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: u64,
    files: Option<Vec<FileInfo>>,
    length: Option<u64>,
    private: Option<u8>
}

impl Info {
    pub fn total_length(&self) -> u64 {
        if let Some(len) = self.length {
            len
        } else if let Some(files) = &self.files {
            files.iter().map(|f| f.length).sum()
        } else {
            0
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct FileInfo {
    path: Vec<String>,
    length: u64
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    // get_tracker_response().await?;
    get_tracker_response()?;
    Ok(())
}

fn parse_torrent(path: &str) -> Result<Torrent, Box<dyn Error>> {
    let content = fs::read(path).map_err(|e| format!("failed to read torrent file '{}': '{}'", path, e))?;
    let torrent: Torrent = serde_bencode::from_bytes(&content).map_err(|e| format!("there was an error parsing the torrent"))?;
    Ok(torrent)
}

fn compute_info_hash(info: Info) -> Result<([u8; 20]), Box<dyn Error>> {
    let info_bytes = serde_bencode::to_bytes(&info).map_err(|e| format!("failed to serialize info dictionary: {}", e))?;
    let mut hasher = Sha1::new();
    hasher.update(&info_bytes);
    Ok(hasher.finalize().into())
}

fn build_tracker_url(announce: &str, info_hash: &[u8; 20], peer_id: String, port: u16, left: u64, uploaded: u64, downloaded: u64, compact: u8, event: Option<&str>) -> Result<String, Box<dyn Error>> {
    // if let Some(announce_list)
    let mut url = Url::parse(announce)?;
    let encoded_hash = percent_encode(
        info_hash, NON_ALPHANUMERIC).to_string();
    {
        let mut query = url.query_pairs_mut();

        query
            .append_pair("info_hash", &encoded_hash)
            .append_pair("peer_id",&peer_id)
            .append_pair("port", &port.to_string())
            .append_pair("left", &left.to_string())
            .append_pair("uploaded", &uploaded.to_string())
            .append_pair("downloaded", &downloaded.to_string())
            .append_pair("compact", &compact.to_string());

        if let Some(event) = event {
            query.append_pair("event", event);
        }
    }

    Ok(url.to_string())
}

fn get_tracker_response() -> Result<String, Box<dyn Error>> {
    let torrent = parse_torrent("Seven Samurai.torrent")?;
    let info_hash = compute_info_hash(torrent.info.clone())?;

    let mut rng  = rand::rng();
    let rand_ = rng.random_range(100_000_000_000u64..=999_999_999_999u64);
    let peer_id = format!("-RS0001-{:012}", rand_);
    assert_eq!(peer_id.len(), 20);

    let left = torrent.info.total_length();
    let tracker_url = build_tracker_url(&torrent.announce.as_ref().unwrap(), &info_hash, peer_id, 6881u16, left, 0, 0, 1, Some("started"))?;

    // let response = reqwest::get(tracker_url).await?.text().await?;

    println!("tracker url: {:?}", torrent);
    Ok(tracker_url)
}