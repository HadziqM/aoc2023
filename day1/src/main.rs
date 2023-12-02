use common::Common;

fn iterate(char:Vec<char>)->usize{
    let mut iter = char.into_iter();
    while let Some(x) = iter.next() {
        if let Some(y) = x.to_digit(10){
            return y as usize;
        }
    }
    0
}


fn main() {
    let mut out:usize = 0;
    for i in &Common::default().whitespace(){
        let value = iterate(i.clone().chars().collect()) * 10 + iterate(i.clone().chars().rev().collect());
        out += value;
    }
    println!("result are {}",out);
}
