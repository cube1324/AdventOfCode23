use std::cmp;
use std::fs::read_to_string;

fn main() {
    // part_1();
    part_2();
}

fn part_2() {
    let mut sum = 0;

    for (i, line) in read_to_string("src/input.txt").unwrap().lines().enumerate() {
        let game = line.split(":").last().unwrap();
        
        sum += get_power(game);
    }

    println!("{}", sum);
}

fn get_power(game: &str) -> i32{
    let draws = game.split(";");

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for draw in draws {
        let (red, green, blue) = get_number(draw); 
        max_red = cmp::max(red, max_red);
        max_green = cmp::max(green, max_green);
        max_blue = cmp::max(blue, max_blue);
    } 
    
    return max_red * max_green * max_blue;
}

fn get_number(draw: &str) -> (i32, i32, i32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let cubes = draw.split(",");
    for cube in cubes {
        let mut split = cube.split(" ");
        split.next();

        let number: i32 = split.next().unwrap().parse().unwrap();

        let color = split.next().unwrap();

        match color {
            "red" => red = number,
            "green" => green = number,
            "blue" => blue = number,
            _ => println!("oops"),
        };
    }

    return (red, green, blue);
}

fn part_1() {
    let mut sum = 0usize;

    for (i, line) in read_to_string("src/input.txt").unwrap().lines().enumerate() {
        let game = line.split(":").last().unwrap();
        
        if is_game_appropriate(game) {
            sum += i + 1;
        }

    }

    println!("{}", sum);
}

fn is_game_appropriate(game: &str) -> bool {
    let draws = game.split(";");

    for draw in draws {
        if !is_draw_appropriate(draw) {
            return false;
        }
    } 
    return true;
}

fn is_draw_appropriate(draw: &str) -> bool {
    // println!("{}", draw);
    let cubes = draw.split(",");
    for cube in cubes {
        let mut split = cube.split(" ");
        split.next();

        let number: i32 = split.next().unwrap().parse().unwrap();

        let color = split.next().unwrap();

        let appropriate =  match color {
            "red" => number < 13,
            "green" => number < 14,
            "blue" => number < 15,
            _ => true,
        };

        if !appropriate {
            return false;
        }

    }

    return true;
}
