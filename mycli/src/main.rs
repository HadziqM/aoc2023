use std::{path::{Path, PathBuf}, process::Command};
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CommandList {
    /// to create new module
    New,
    /// to run existing module
    Run,
    /// to asnwer part
    Answer,
}


#[derive(Parser,Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI{
    ///arguments
    #[arg(value_enum)]
    command:CommandList,
    /// day of challange
    day:u8,

    ///only download input
    #[arg(short,long)]
    input:bool,
    ///only download input
    #[arg(short,long)]
    question:bool,

    ///part for answer
    #[arg(short,long)]
    part:Option<u8>,

    ///the answer
    #[arg(short,long)]
    answer:Option<String>
}

#[derive(Debug,Serialize,Deserialize)]
struct Workplace {
    workspace: Workspace
}
#[derive(Debug,Serialize,Deserialize)]
struct Workspace {
    members: Vec<String>,
    resolver:String
}

impl Default for Workplace {
    fn default() -> Self {
        toml::from_str(&std::fs::read_to_string(Self::path()).expect("cant find Cargo.toml in worksplace"))
            .expect("the struct are invalid")
    }
}

#[derive(Serialize,Deserialize)]
struct Session{
    cookie:String
}

impl Default for Session {
    fn default() -> Self {
        let path = Path::new(".").join("session.json");
        serde_json::from_slice(&std::fs::read(path).expect("cant get json")).expect("json didnt match")
    }
}

impl Workplace {
    fn path()-> PathBuf {
        Path::new(".").join("Cargo.toml")
    }
    fn add_member(&mut self,member:String) {
        self.workspace.members.push(member)
    }
    fn save(&self) {
        let content = toml::to_string_pretty(&self).expect("cant serialize the toml struct");
        std::fs::write(Self::path(), &content).expect("cant write into file");
    }
}


impl CLI {
    fn url(&self,inp:&str)-> String {
        if inp == "" {
            return format!("https://adventofcode.com/2023/day/{}",self.day);
        }
        format!("https://adventofcode.com/2023/day/{}/{inp}",self.day)
    }
    fn challange(&self) {
        println!("https://adventofcode.com/2023/day/{}",self.day);
    }
    fn path(&self) -> PathBuf {
        Path::new(".").join(&format!("day{}",self.day))
    }
    fn name(&self) -> String {
        format!("day{}",self.day)
    }
    async fn download_input(&self) {
        let client = reqwest::Client::new();
        let ses = Session::default();
        let bytes = client.get(&self.url("input")).header("Cookie", &format!("session={}",ses.cookie)).send()
            .await.expect("url not found").bytes().await.expect("its not bytes");
        std::fs::write(&self.path().join("input.txt"), &bytes).expect("cant crate file");
    }
    async fn download_question(&self) {
        let client = reqwest::Client::new();
        let ses = Session::default();
        let url = self.url("");
        let text = client.get(&url).header("Cookie", &format!("session={}",ses.cookie)).send()
            .await.expect("url not found").text().await.expect("its not bytes");
        let html = Html::parse_document(&text);
        let body = html.select(&Selector::parse("body").unwrap())
            .next().expect("cant find body");
        let main = body.select(&Selector::parse("main").unwrap())
            .next().expect("cant find main");
        for (i,article) in main.select(&Selector::parse("article").unwrap()).enumerate(){
            let part = i+1;
            let markdown = html2md::parse_html(&article.inner_html());
            std::fs::write(&self.path().join(&format!("part{part}.md")), markdown.as_bytes()).expect("cant crate file");
        }

    }
    fn create_main_file(&self) {
        let code = r#"
use common::Common;


fn main() {
    let puzzle =  Common::default();
    // you can make your solution here
    // use puzle.input() to get the input string;
    

    // to automatically answer the puzzle
    // puzzle.answer(1,myanswer)
}

        "#;
        let path = self.path().join("src").join("main.rs");
        std::fs::write(path, code.to_string().as_bytes())
            .expect("cant write code");
        let ident = self.path().join("ident.idt");
        std::fs::write(ident, format!("{}",self.day).as_bytes())
            .expect("cant write code")
    }
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    match cli.command{
        CommandList::New => {
            if !cli.input && !cli.question {
                let mut work = Workplace::default();
                work.add_member(cli.name());
                work.save();
                let cmd = Command::new("sh")
                        .args(&["-c",&format!("cargo init day{}",cli.day)])
                        .spawn().expect("cant run subprocess");
                cmd.wait_with_output().expect("the subprocess cant stop");
                let cmd = Command::new("sh")
                        .args(&["-c",&format!("cargo add common --path ./common --package day{}",cli.day)])
                        .spawn().expect("cant run subprocess");
                cmd.wait_with_output().expect("the subprocess cant stop");
                cli.create_main_file();
                cli.download_input().await;
                cli.download_question().await;
            }else if cli.input {
                cli.download_input().await;
            }else if cli.question {
                cli.download_question().await;
            }
            cli.challange();

        }
        CommandList::Run => {
            let day = cli.day;
            let cmd = Command::new("sh")
                    .args(&["-c",&format!("cd day{day}&& cargo run")])
                    .spawn().expect("cant run subprocess");
            cmd.wait_with_output().expect("the subprocess cant stop");
        }
        CommandList::Answer => {
            let day = cli.day;
            if let Some(part) = cli.part {
                if let Some(answer) = cli.answer{
                    let client = reqwest::Client::new();
                    let ses  = Session::default();
                    let res = client.post(&format!("https://adventofcode.com/2023/day/{day}/answer"))
                        .header(reqwest::header::COOKIE, &format!("session={}",ses.cookie))
                        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                        .body(format!("level={part}&answer={answer}"))
                        .send().await.expect("cant reach the web, check the cookie");
                    let text = res.text().await.unwrap();
                    let html = Html::parse_document(&text);
                    let body = html.select(&Selector::parse("body").unwrap())
                        .next().expect("cant find body inside html");
                    let main = body.select(&Selector::parse("main").unwrap())
                        .next().expect("cant find main inside html");
                    let outcome = main.inner_html();
                    if outcome.contains("That's the right answer") {
                        println!("you nailed it");
                    } else if outcome.contains("That's not the right answer") {
                        println!("answer are incorrect");
                    } else if outcome.contains("You gave an answer too recently") {
                        println!("need to wait a min");
                    } else if outcome
                        .contains("You don't seem to be solving the right level")
                    {
                        println!("the level/part are incorrect or already answered");
                    } else {
                        println!("idk really");
                    }
                }else {
                    println!("specify part with --part 1 and --answer x");
                }
            }else {
                println!("specify part with --part 1 and --answer x");
            }
        }
    }
}
