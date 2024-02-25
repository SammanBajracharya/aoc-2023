use std::{process, fs};

pub fn extract_content(args: &[String]) -> String {
    if args.len() != 2 {
        eprintln!("Not Enough Arguments");
        process::exit(0);
    }
    
    fs::read_to_string(&args[1]).unwrap_or_else(|err| {
        eprintln!("Error Reading File: {}", err);
        process::exit(0);
    })
}

pub fn contain_cube(contents: &str) -> u16 {
    let mut ans: u16 = 0;

    for (id, line) in contents.lines().enumerate() {
        let parts: Vec<&str> = line
                .split(": ")
                .collect();

        let valid: bool = validate_cubes(parts.get(1).unwrap_or(&""));

        if valid {
            ans += (id+1) as u16;
        }
    }

    ans
}

fn validate_cubes(parts: &str) -> bool {
    let cubes: Vec<&str> = parts.split(";").collect();
    let mut temp_str: String;
    let mut temp_arr: Vec<&str>;
    let mut valid: bool = false;

    let mut red: u16 = 0; 
    let mut green: u16 = 0; 
    let mut blue: u16 = 0; 

    let mut max_red: u16 = 0;
    let mut max_green: u16 = 0;
    let mut max_blue: u16 = 0;

    for i in 0..cubes.len() {
        temp_str = cubes[i].replace(",", "");
        temp_arr = temp_str.trim().split_whitespace().collect();
        
        for j in 0..temp_arr.len() {
            match temp_arr[j] {
                "red" => red = temp_arr[j-1].parse::<u16>().unwrap_or(0),
                "green" => green = temp_arr[j-1].parse::<u16>().unwrap_or(0),
                "blue" => blue = temp_arr[j-1].parse::<u16>().unwrap_or(0),
                _ => {},
            }
        }

        if 
            red <= 12 &&
            green <= 13 &&
            blue <= 14
        {
            valid = true;
        } else {
            return false;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_contain_cube() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let results = contain_cube(&contents);
        assert_eq!(8, results);
    }
}
