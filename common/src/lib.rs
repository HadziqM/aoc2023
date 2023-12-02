use std::path::Path;
use regex::Regex;
use serde::Deserialize;

pub struct Common{
    pub input:String
}


#[derive(Deserialize)]
struct Session{
    cookie:String
}

impl Default for Session {
    fn default() -> Self {
        let path = Path::new("..").join("session.json");
        serde_json::from_slice(&std::fs::read(path).expect("cant get json")).expect("json didnt match")
    }
}

impl Default for Common {
    fn default() -> Self {
        let path = Path::new(".").join("./input.txt");
        let input = std::fs::read_to_string(path).expect("cant find input");
        Self { input }
    }
}

impl std::ops::Deref for Common {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl Common {
    pub fn input(&self) -> String {
        self.input.clone()
    }
    pub fn whitespace(&self) -> Vec<String>{
        self.input().split_whitespace().map(|x|x.to_owned()).collect()
    }
    /// answer puzzle automatically if no part then use 1
    pub fn answer(day:u8,part:u8,answer:String) {
        tokio::spawn(async move{
            Self::answer_async(day, part, answer).await
        });
    }
    async fn answer_async(day:u8,part:u8,answer:String) {
        let client = reqwest::Client::new();
        let ses  = Session::default();
        let res = client.post(&format!("https://adventofcode.com/2023/day/{day}/answer"))
            .header(reqwest::header::COOKIE, &format!("session={}",ses.cookie))
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!("level={part}&answer={answer}"))
            .send().await.expect("cant reach the web, check the cookie");
        let outcome = Regex::new("r(?i)(?s)<main>(?P<main>.*)</main>").unwrap()
            .captures(&res.text().await.unwrap())
            .expect("something wrong with regex")
            .name("main").unwrap()
            .as_str().to_string();
        if outcome.contains("That's the right answer") {
            println!("You nailed it");
        } else if outcome.contains("That's not the right answer") {
            println!("The answer is incorrect");
        } else if outcome.contains("You gave an answer too recently") {
            println!("get rate limited, try again after 1 min");
        } else if outcome
            .contains("You don't seem to be solving the right level")
        {
            println!("the part are incorrect");
        } else {
            println!("idk really....");
        }
    }
}
