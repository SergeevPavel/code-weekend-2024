use std::{fs::File, path::Path};

use reqwest::{Method, Url};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub base_speed: i32,
    pub base_power: i32,
    pub base_range: i32,
    pub level_speed_coeff: i32,
    pub level_power_coeff: i32,
    pub level_range_coeff: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Monster {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub gold: i32,
    pub exp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub hero: Hero,
    pub start_x: i32,
    pub start_y: i32,
    pub width: i32,
    pub height: i32,
    pub num_turns: i32,
    pub monsters: Vec<Monster>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "move")]
    Move {
        comment: Option<String>,
        target_x: i32,
        target_y: i32,
    },
    #[serde(rename = "attack")]
    Attack {
        comment: Option<String>,
        target_id: usize
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Solution {
    pub moves: Vec<Command> 
}

pub fn read_task(test_id: u32) -> Task {
    let path = Path::new("data/tests").join(test_name(test_id));
    println!("Open: {:?}", path);
    let file = File::open(path).unwrap();
    let task: Task = serde_json::from_reader(file).unwrap();
    task
}

const API_URL: &str = "https://codeweekend.dev:3721/api/";
const FILE_URL: &str = "https://codeweekend.dev:81/";
const TOKEN: &str = ""; // put your token here

pub fn get_scoreboard() {
    let client = Client::new();
    let url = Url::parse(API_URL).unwrap().join("scoreboard").unwrap();
    let resp = client.request(Method::GET, url).bearer_auth(TOKEN).send().unwrap();
    let result: serde_json::Value = resp.json().unwrap();
    println!("Result: {:?}", result);
}

fn test_name(test_id: u32) -> String {
    format!("{:03}.json", test_id)
}

pub fn get_test(test_id: u32) -> Task {
    let client = Client::new();
    let url = Url::parse(FILE_URL).unwrap().join(&test_name(test_id)).unwrap();
    let resp = client.request(Method::GET, url).bearer_auth(TOKEN).send().unwrap();
    resp.json().unwrap()
}

pub fn submit(test_id: u32, solution: &Solution) {
    let client = Client::new();
    let url = Url::parse(API_URL).unwrap()
        .join("submit/").unwrap()
        .join(&test_id.to_string()).unwrap();
    let solution_str = serde_json::to_string(solution).unwrap();
    let form = reqwest::blocking::multipart::Form::new()
        .text("file", solution_str);
    let resp = client.request(Method::POST, url)
        .bearer_auth(TOKEN)
        .multipart(form)
        .send().unwrap();
    eprintln!("Resp: {:?}", resp);
    eprintln!("Resp text: {:?}", resp.text());
}