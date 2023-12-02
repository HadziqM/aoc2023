use std::{path::Path, process::Command};

pub struct Common{
    pub input:String,
    day:u8
}



impl std::ops::Deref for Common {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl Common {
    pub fn day(day:u8) -> Self {
        let path = Path::new(".").join("./input.txt");
        let input = std::fs::read_to_string(path).expect("cant find input");
        Self { input,day }
    }
    pub fn input(&self) -> String {
        self.input.clone()
    }
    pub fn whitespace(&self) -> Vec<String>{
        self.input().split_whitespace().map(|x|x.to_owned()).collect()
    }
    /// answer puzzle automatically if no part then use 1 (first part)
    pub fn answer(&self,part:u8,answer:String) {
        let cmd = Command::new("sh")
                .args(&["-c",&format!("cd ..&&mycli answer {} --part {part} --answer {answer}",self.day)])
                .spawn().expect("cant run subprocess");
        cmd.wait_with_output().expect("the subprocess cant stop");
    }
}
