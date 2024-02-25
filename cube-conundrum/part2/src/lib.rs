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

pub fn contain_cube(contents: &str) -> usize {
    let mut ans = 0;

    let mut red_list: Vec<usize> = Vec::new();
    let mut green_list: Vec<usize> = Vec::new();
    let mut blue_list: Vec<usize> = Vec::new();

    let mut _red: usize = 0;
    let mut _green: usize = 0;
    let mut _blue: usize = 0;

    for line in contents.lines() {
        let parts: Vec<&str> = line
                .split(": ")
                .collect();

        let temp_str: &str = &parts[1].replace(",", "").replace(";", ""); 
        let temp_arr: Vec<&str> = temp_str.split_whitespace().collect();

        for i in 1..temp_arr.len() {
            match temp_arr[i] {
                "red" => red_list.push(temp_arr[i-1].parse::<usize>().unwrap_or(0)),
                "green" => green_list.push(temp_arr[i-1].parse::<usize>().unwrap_or(0)),
                "blue" => blue_list.push(temp_arr[i-1].parse::<usize>().unwrap_or(0)),
                _ => {},
            }
        }

        _red = *red_list.iter().max().unwrap();
        _green = *green_list.iter().max().unwrap();
        _blue = *blue_list.iter().max().unwrap();

        red_list.clear();
        green_list.clear();
        blue_list.clear();

        ans += _red * _green * _blue;
    }

    ans
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
        assert_eq!(2286, results);
    }
}
