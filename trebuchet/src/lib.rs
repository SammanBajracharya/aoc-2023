use std::{collections::HashMap, fs, process};

pub fn extract_content(args: &[String]) -> String {
    if args.len() != 2 {
        eprintln!("Not Enough Argument");
        process::exit(1);
    }

    let contents = fs::read_to_string(&args[1]).unwrap();
    contents
}

pub fn get_sum(contents: &str) -> u16 {
    let results: Vec<u16> = extract_numeric_values(&contents);
    results.iter().sum()
}

fn extract_numeric_values(contents: &str) -> Vec<u16> {
    let mut arr: Vec<u16> = Vec::new();
    let mut result: Vec<u16> = Vec::new();
    let mut temp: u16;

    let value_list: HashMap<&str, u16> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let char_list: [char; 7] = ['z', 'o', 't', 'f', 's', 'e', 'n']; 

    for line in contents.lines() {
        for (pos, c) in line.chars().enumerate() {
            if c.is_numeric() {
                arr.push(c.to_digit(10).unwrap() as u16);
            } else {
                if char_list.contains(&c) {
                    for i in 3..=5 {
                        if let Some(eqi_val) = line.get(pos..pos+i) {
                            if let Some(&val) = value_list.get(eqi_val) {
                                arr.push(val);
                            }
                        }
                        
                    }
                }
            }
        }

        temp = arr[0]*10 + arr[arr.len()-1];
        result.push(temp);
        arr.clear();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numeric_values() {
        let contents = "two934seven1
8825eightknfv
sevenoneqbfzntsix55
foursqpqvv192rdrbtcccfourone
9jpzhpxqthreelmrnlhfqmn4
onedpsckg3xdhmgtsixthreefivejlncszkxeight
4twofour
7eighttwo17fournsmrznntgjrdpkdjvx";
        let results = extract_numeric_values(&contents);
        assert_eq!(vec![21, 88, 75, 41, 94, 18, 44, 74], results);
    }

    #[test]
    fn test_get_sum() {
        let contents = "two934seven1
8825eightknfv
sevenoneqbfzntsix55
foursqpqvv192rdrbtcccfourone
9jpzhpxqthreelmrnlhfqmn4
onedpsckg3xdhmgtsixthreefivejlncszkxeight
4twofour
7eighttwo17fournsmrznntgjrdpkdjvx";
        let results: u16 = get_sum(&contents);
        assert_eq!(455, results);
    }
}
