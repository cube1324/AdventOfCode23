use std::fs::read_to_string;


fn main() {
    // let mut result = Vec::new();

    let mut s;
    let mut e;

    let mut sum = 0;

    for mut line in read_to_string("src/test.txt").unwrap().lines() {
        // let mut temp = str::replace(line, "one", "1");
        // temp = str::replace(&temp, "two", "2");
        // temp = str::replace(&temp, "three", "3");
        // temp = str::replace(&temp, "four", "4");
        // temp = str::replace(&temp, "five", "5");
        // temp = str::replace(&temp, "six", "6");
        // temp = str::replace(&temp, "seven", "7");
        // temp = str::replace(&temp, "eight", "8");
        // temp = str::replace(&temp, "nine", "9");

        s = get_first(&line.chars().collect());
        e = get_first(&line.chars().rev().collect());

        println!("{} {} {}", line, s, e);
        
        sum += format!("{}{}", s, e).parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

fn get_first(charset: &Vec<char>) -> char {
    for el in charset {
        if el.is_digit(10){
            return *el;
        }
    }
    return '0';
}
