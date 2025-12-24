use std::io;

pub fn ex_1() -> Result<f64, String> {
    let mut temp: String = String::new();
    let mut temp_type: String = String::new();

    println!("Celsius to Fahrenheit and Fahrenheit to Celsius converter.");

    println!("Please enter the temperature.");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to get the temperature, please try again.");

    println!("Please specify the type: 'F' for Fahrenheit or 'C' for Celsius.");
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to get the temperature type, please try again.");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Please enter valid temperature value.".to_string());
        }
    };

    let temp_type: char = match temp_type.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            return Err("Please enter valid temperature type.".to_string());
        }
    };

    match temp_type {
        'F' => Ok(((temp - 32.0) * 5.0) / 9.0),
        'C' => Ok(((temp * 9.0) / 5.0) + 32.0),
        _ => Err("Please enter valid temperature type.".to_string()),
    }
}

pub fn ex_2(n: i32) -> Result<i32, String> {
    // generate nth fibonacci number
    // seq: 0 1 1 2 3 5 8 .....
    if n <= 0 {
        return Err("Please enter the correct value of n".to_string());
    }

    if n == 1 {
        return Ok(0);
    }

    if n == 2 {
        return Ok(1);
    }

    let a = match ex_2(n - 1) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let b = match ex_2(n - 2) {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    Ok(a + b)
}

pub fn ex_3() {
    let last_line_of_every_verse: &str = "A partridge in a pear tree";

    let mut day: usize = 1;
    let days_in_english: [&str; 13] = [
        "", "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let everyday_verses = [
        "",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    while day < 13 {
        println!(
            "On the {} day of Christmas, my true love sent to me,",
            days_in_english[day]
        );
        let mut i: usize = 1;
        while i < day {
            println!("{},", everyday_verses[i]);
            i += 1;
        }
        println!("{last_line_of_every_verse},\n");
        day += 1;
    }
}
