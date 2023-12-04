
use std::{fs::read_to_string};


fn main() {
    let cards = read_to_string("src/input.txt").unwrap();

    let mut instances = vec![1; cards.lines().count()];

    for (i, card) in &mut cards.lines().enumerate() {
        let all_numbers = card.split(": ").last().unwrap();

        let numbers: Vec<&str> = all_numbers.split(" | ").collect();

        let my_numbers = numbers[0].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<u16>().unwrap());

        let winning_numbers: Vec<u16> = numbers[1].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<u16>().unwrap()).collect();

        let mut matches = 0;
        for number in my_numbers {
            
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        
        for j in 0 .. matches {
            instances[i + j + 1] += instances[i]; 
        }
    }

    let sum: i32 = instances.iter().sum();
    println!("{}", sum)


}


fn main1() {
    let cards = read_to_string("src/input.txt").unwrap();


    let mut sum = 0;

    let base: i32 = 2;


    for card in &mut cards.lines() {
        let all_numbers = card.split(": ").last().unwrap();

        let numbers: Vec<&str> = all_numbers.split(" | ").collect();

        let my_numbers = numbers[0].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<u16>().unwrap());

        let winning_numbers: Vec<u16> = numbers[1].split(" ").filter(|&x| !x.is_empty()).map(|x| x.parse::<u16>().unwrap()).collect();

        let mut matches = 0;
        for number in my_numbers {
            
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }
        if matches > 0 {
            sum += base.pow(matches - 1);
        }

    }

    println!("{}", sum)


}
