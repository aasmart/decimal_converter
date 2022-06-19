pub struct Config {
    pub conversion: String,
    pub number: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments");
        }

        let conversion = match args[1].clone().to_lowercase().as_str() {
            "binary" => args[1].clone(),
            "hexadecimal" => args[1].clone(),
            &_ => return Err("invalid conversion type")
        };

        let num_result = args[2].clone().parse::<i32>();
        let number = match num_result {
            Ok(number) => number,
            Err(_e) => return Err("input must be an integer")
        };

        if number < 0 {
            return Err("input must be positive");
        }

        Ok(Self {conversion, number})
    }
}

pub fn run(config: &Config) -> Result<(), &'static str>{
    let result = match config.conversion.as_str() {
        "binary" => convert_to_binary(config.number),
        "hexadecimal" => convert_to_hex(config.number),
        _ => return Err("what the heck did you break!?")
    };
    println!("The number {} in binary is {}", config.number, result);

    Ok(())
}

fn convert_to_binary(mut number: i32) -> String {
    let mut result: Vec<String> = Vec::new();
    while number > 0 {
        let remainder = number % 2;
        number /= 2;

        result.insert(0, remainder.to_string());
    }

    let mut result_str: String = String::new();
    for str in &result {
        result_str.push(str.parse::<char>().unwrap());
    }

    result_str
}

fn convert_to_hex(mut number: i32) -> String {
    let mut result: Vec<String> = Vec::new();
    while number > 0 {
        let remainder = number % 16;
        number /= 16;

        let digit: String = match remainder {
            10 => String::from("A"),
            11 => String::from("B"),
            12 => String::from("C"),
            13 => String::from("D"),
            14 => String::from("E"),
            15 => String::from("F"),
            _ => remainder.to_string()
        };

        result.insert(0, digit);
    }

    let mut result_str: String = String::new();
    for str in &result {
        result_str.push(str.parse::<char>().unwrap());
    }

    result_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_to_binary() {
        assert_eq!(convert_to_binary(10), String::from("1010"));
    }

    #[test]
    fn five_to_binary() {
        assert_eq!(convert_to_binary(5), String::from("101"));
    }

    #[test]
    fn one_hundred_to_binary() {
        assert_eq!(convert_to_binary(100), String::from("1100100"));
    }

    #[test]
    fn fifteen_as_hex() {
        assert_eq!(convert_to_hex(15), String::from("F"));
    }

    #[test]
    fn fifty_as_hex() {
        assert_eq!(convert_to_hex(50), String::from("32"));
    }
}

