use csv::Reader;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result};

#[derive(Serialize, Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]

struct Player {
    name : String,
    position : String,
    #[serde(rename = "DOB")]
    dob : String,
    #[serde(rename = "Nationality")]
    nationality : String,
    #[serde(rename = "Kit Number")]
    kit:u8,
}

pub fn process_csv(intput: &str, output:&str)->Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record:Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}