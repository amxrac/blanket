use serde_derive::{Serialize, Deserialize};
use serde_bencode::{self, from_bytes};
use serde;
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
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct FileInfo {
    path: Vec<String>,
    length: u64
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Malena.torrent";
    let torrent_info: Torrent = serde_bencode::from_bytes(&fs::read(file_path)?).map_err(|e| format!("there was an error parsing the torrent"))?;

    println!("torrent info: {:?}", torrent_info);
    Ok(())
}