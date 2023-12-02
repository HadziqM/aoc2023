
use common::Common;

#[derive(Clone)]
enum Cubes{
    Red(usize),
    Green(usize),
    Blue(usize)
}
impl Cubes{
    fn parse(inp:&str) -> Self {
        let mut x = inp.split(" ");
        let num = x.next().unwrap().parse::<usize>().unwrap();
        match x.next().unwrap() {
            "blue" => Self::Blue(num),
            "red" => Self::Red(num),
            "green" => Self::Green(num),
            _ => std::panic!("undefined {inp}")
        }
    }
    fn validate(inp:&Vec<Self>) -> bool {
        for i in inp{
            let res = match i {
                Self::Green(x) => x <= &13,
                Self::Red(x) => x <= &12,
                Self::Blue(x) => x <= &14,
            };
            if !res {
                return false;
            }
        }
        true
    }
    fn min(inp:&Vec<Self>) -> usize {
        let mut red = vec![];
        let mut green = vec![];
        let mut blue = vec![];
        for i in inp{
            match i.to_owned() {
                Self::Green(x) => green.push(x),
                Self::Red(x) => red.push(x),
                Self::Blue(x) => blue.push(x),
            };
        }
        red.iter().max().unwrap() * green.iter().max().unwrap() * blue.iter().max().unwrap()
    }
}


fn parsing(line:&str) -> (bool,usize) {
    if let Some(data) = line.split(":").nth(1){
        let mut cont = vec![];
        for x in data.split(";") {
            for z in x.split(",") {
                cont.push(Cubes::parse(z.trim()));
            }
        }
        return (Cubes::validate(&cont),Cubes::min(&cont));
    }
    (false,0)
}

fn main() {
    let puzzle =  Common::default();
    let mut answer1:usize = 0;
    let mut answer2:usize = 0;
    for (index,data) in puzzle.split("\n").enumerate() {
        let (x,y) = parsing(data.trim());
        if x {
            answer1 += index + 1;
        }
        answer2 += y;
    }

    println!("my answer1 = {answer1} asnwer2 = {answer2}");
    puzzle.answer(2,answer2.to_string())
}
