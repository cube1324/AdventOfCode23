use std::fs::read_to_string;


fn main() {
    let binding = read_to_string("src/input.txt").unwrap();
    let mut lines : Vec<String> = binding.lines().map(|line| line.to_string()).collect();

    for line in &mut lines {
        line.push('.');
        line.insert(0, '.');
        // println!("{}", line);
    }

    let len_line = lines[0].len();

    lines.push(str::repeat(".", len_line));
    lines.insert(0, str::repeat(".", len_line));

    let data: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let num = check_for_numbers(&data);

    println!("{}", num);

}


fn check_for_numbers(data: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;

    let len_line = data[0].len();
    let len = data.len();

    let mut current_number = "".to_string();
    let mut is_part = false;

    for i in 1..len - 1 {
        for j in 1 .. len_line - 1 {
            if data[i][j].is_digit(10) {
                current_number.push(data[i][j]);

                is_part = is_part | check_for_symbol(&data, i as i32, j as i32);

                if !data[i][j + 1].is_digit(10) {
                    if is_part{
                        let number: usize = current_number.parse().unwrap();
                        sum += number;
                    }
                    current_number = "".to_string();
                    is_part = false;
                }
            }

        }
    }
    return sum;
    
}

fn check_for_symbol(data: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    for (x, y) in vec![(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)] {
        
        if !data[(i + x) as usize][(j + y) as usize].is_digit(10) & (data[(i + x) as usize][(j + y) as usize] != '.') {
            return true;
        }
        
    }
    return false;
}