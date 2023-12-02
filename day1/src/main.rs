use common::Common;


pub fn get_num_string(char:Vec<char>,string_num:bool) -> u32 {
    let mut out = vec![];
    let mut checker = vec![];
    let mut iter = char.into_iter();
    while let Some(x) = iter.next() {
        if let Some(y) = x.to_digit(10){
            out.push(y);
            checker.clear();
        }else if string_num {
            checker.push(x);
            let word:String = checker.iter().collect();
            if word.ends_with("one"){
                out.push(1);
                checker.clear();
            }else if word.ends_with("two"){
                out.push(2);
                checker.clear();
            }else if word.ends_with("three"){
                out.push(3);
                checker.clear();
            }else if word.ends_with("four"){
                out.push(4);
                checker.clear();
            }else if word.ends_with("five"){
                out.push(5);
                checker.clear();
            }else if word.ends_with("six"){
                out.push(6);
                checker.clear();
            }else if word.ends_with("seven"){
                out.push(7);
                checker.clear();
            }else if word.ends_with("eight"){
                out.push(8);
                checker.clear();
            }else if word.ends_with("nine"){
                out.push(9);
                checker.clear();
            }
        }
    }
    if out.len() != 1 {
        out.first().unwrap() * 10 + out.last().unwrap()
    }else {
        out[0] * 10 + out[0]
    }
}

fn main() {
    let puzzle =  Common::day(1);
    let mut answer1 = 0;
    let mut answer2 = 0;
    for i in &puzzle.whitespace(){
        answer1 += get_num_string(i.clone().chars().collect(), false);
        answer2 += get_num_string(i.chars().collect(), true);
    }

    println!("my answer part1 = {answer1} part2 = {answer2}");

    // puzzle.answer(1, answer1.to_string());
    // puzzle.answer(2, answer2.to_string());

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn part2() {
        let puzzle = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "#.to_string();
        let mut answer2 = 0;
        for i in puzzle.split_whitespace(){
            answer2 += get_num_string(i.chars().collect(), true);
        }
        assert_eq!(answer2,281 );
    }
}
