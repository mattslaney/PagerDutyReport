extern crate console_error_panic_hook;

use std::panic;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::Deserialize;

// #[derive(Deserialize, Debug, Clone)]
// pub struct Record {
//     number: String,
//     pub title: String,
//     urgency: String,
//     priority: String,
//     service: String,
//     created_at: String,
//     resolved_by: String,
//     auto_resolved: String,
//     responders: String,
//     tta: String,
//     ttr: String,
//     re: String,
//     escalations: String
// }

#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    number: String,
    incident_id: String,
    responder: String,
    team: String,
    pub title: String,
    service: String,
    urgency: String,
    priority: String,
}

#[wasm_bindgen]
extern {
    pub fn print_line(line: &str);
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn info(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[wasm_bindgen]
pub fn alert_rs(msg: &str) {
    log(msg);
}

#[wasm_bindgen]
pub fn print_file(data: &str) {
    log(data);
}

#[wasm_bindgen]
pub fn process_csv(data: &str) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let records = get_records(data);
    let records = group_records(records, 0.5);
    debug(&format!("title: {:?}", records));
    let table = gen_table(records);
    print_line(&table);
}

fn get_records(data: &str) -> Vec<Record> {
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    let mut count = 0;
    let mut records: Vec<Record> = Vec::new();

    for record in reader.records() {
        count = count + 1;
        match record {
            Ok(record) => {
                let record: Result<Record, csv::Error> = record.deserialize(None);
                match record {
                    Ok(record) => {
                        records.push(record);
                    }
                    Err(e) => {
                        error(&format!("Failed to get record {}: {:?}", count, e));
                    }
                }
            }
            Err(e) => {
                error(&format!("Failed to get record {}: {:?}", count, e));
            }
        }
    }

    log(&format!("{count}"));

    records
}

fn group_records(records: Vec<Record>, threshold: f64) -> HashMap::<String, Vec<Record>> {
    let mut grouped_records: HashMap<String, Vec<Record>> = HashMap::new();

    for record in records {
        let mut found_group = false;
        for (representative, group) in &mut grouped_records {
            if let Some(similarity) = group.first().map(|r| calculate_similarity(&record.title, &r.title)) {
                if similarity >= threshold {
                    group.push(record.clone());
                    found_group = true;
                    break;
                }
            }
        }
        if !found_group {
            grouped_records.insert(record.title.clone(), vec![record]);
        }
    }

    for (key, value) in &grouped_records {
        debug(&format!("# {} - {} - {:?}", value.len(), key, value));
    }

    grouped_records
}

fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let len = s1.len().max(s2.len());
    let matches = s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 == c2).count();
    matches as f64 / len as f64
}

fn gen_table(grouped_records: HashMap<String, Vec<Record>>) -> String {
    let mut table = String::new();
    table.push_str("<table class=\"incidents\">");
    table.push_str("<tr><th>Count</th><th>Desc</th><th>Example</th>");
    
    for (key, value) in grouped_records {
        // let mut key = key;
        // key.truncate(50);
        let common_str = longest_common_substring(&value.iter().map(|r| r.title.as_str()).collect::<Vec<_>>());
        let common_mask = common_substring_mask(&value.iter().map(|r| r.title.as_str()).collect::<Vec<_>>());
        table.push_str("<tr><td>");
        table.push_str(&format!("{}",value.len()));
        table.push_str("</td><td>");
        table.push_str(&common_mask);
        table.push_str("</td><td>");
        table.push_str(&key);
        table.push_str("</td></tr>");
    }
    
    table.push_str("</table>");
    table
}

fn common_substring_mask(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    } else if strings.len() == 1 {
        return strings[0].to_string();
    }

    let mut mask = String::new();

    let first_string = strings[0];
    let second_string = strings.get(1).unwrap_or(&"");

    for (c1, c2) in first_string.chars().zip(second_string.chars()) {
        if c1 == c2 {
            mask.push(c1);
        } else {
            mask.push(' ');
        }
    }

    mask
}

fn longest_common_substring(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let shortest = strings.iter().min_by_key(|s| s.len()).unwrap();
    let mut longest_common_substring = String::new();

    'outer: for i in 0..shortest.len() {
        for j in (i + 1)..=shortest.len() {
            let substring = &shortest[i..j];
            if strings.iter().all(|s| s.contains(substring)) {
                if substring.len() > longest_common_substring.len() {
                    longest_common_substring = substring.to_string();
                }
            } else {
                continue 'outer;
            }
        }
    }

    longest_common_substring
}
