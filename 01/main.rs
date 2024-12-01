fn get_index_of_char(string: &str, c: char) -> Option<usize>
{
    for (i, _c) in string.chars().enumerate()
    {
        if _c == c
        {
            return Some(i);
        }
    }
    return None;
}

fn get_second_digit_index(string: &str) -> usize
{
    let mut sep_found = false;
    for (i, c) in string.chars().enumerate()
    {
        if c == ' '
        {
            sep_found = true;
        }
        if c.is_digit(10) && sep_found
        {
            return i;
        }
    }

    panic!("No other digit in string");
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    let path = "input.txt";
    let mut result: u64 = 0;

    for line in std::fs::read_to_string(path).unwrap().lines()
    {
        match get_index_of_char(line, ' ') {
            None => break,
            Some(i) => {
                v1.push(line[0..i].parse::<i32>().unwrap());
                let j = get_second_digit_index(line);
                v2.push(line[j..].parse::<i32>().unwrap());
            }
        }
    }

    v1.sort();
    v2.sort();

    for (i, p1) in v1.iter().enumerate()
    {
        result += (p1 - v2[i]).abs() as u64;
    }

    println!("Result: {}", result);
}
