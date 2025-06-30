use serde_derive::{Serialize, Deserialize};
use serde_bencode::{self, from_bytes};
use serde;
use sha1::{Digest, Sha1};
use std::{any::{self, Any}, fs, string};
use std::collections::HashMap;
use chrono::{NaiveDate, DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::error::Error;


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


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct  Info {
    name: String,
    // #[serde(with = "serde_bytes")]
    // pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: u64,
    files: Option<Vec<FileInfo>>,
    lengt: Option<u64>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct FileInfo {
    path: Vec<String>,
    length: u64
}

fn main() -> Result<(), Box<dyn Error>> {
    let torrent = parse_torrent("Seven Samurai.torrent");
    let info_hash = compute_info_hash(torrent.unwrap().info)?;
    println!("info hash: {:?}", info_hash);
    println!("info hash hex: {:?}", hex::encode(info_hash));    
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