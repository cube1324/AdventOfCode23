use std::fs::read_to_string;


fn main() {
    // let mut result = Vec::new();

    let mut s;
    let mut e;

    let mut sum = 0;

    let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let words_reversed = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

    for line in read_to_string("src/input.txt").unwrap().lines() {

        e = get_last(&mut line.to_string(), &words);
        s = get_last(&mut line.chars().rev().collect(), &words_reversed);

        sum += format!("{}{}", s, e).parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

fn get_last(line: &mut String, words: &Vec<&str>) -> char {
    while line.len() > 0{
        for (i, number) in words.iter().enumerate() {
            if line.ends_with(*number) {
                return char::from_digit((i + 1) as u32, 10).unwrap();
            }
        }
        let el = line.pop().unwrap();

        if el.is_digit(10){
            return el;
        }
    }
    return '0';
}
