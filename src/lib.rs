// 문서용 설명 작성
//! # RPN Calc
//! Reverse Polish notation (RPN) Calc.
//! # Example
//! ```
//! let src = String::from("1 2 + 3 * ");
//! let a = fprice::eval(src).unwrap();
//! println!("{}", a);
//! ```

fn fmt_num_i32_str(number: i32) -> String {
    let mut formatted_number = String::new();
    let mut count = 0;
    let number_convert = number.to_string();

    // Iterate through the characters of the number from right to left
    for c in number_convert.chars().rev() {
        if count == 3 {
            formatted_number.push(',');
            count = 0;
        }
        formatted_number.push(c);
        count += 1;
    }

    // Reverse the formatted number back to the original order
    formatted_number.chars().rev().collect()
}

fn fmt_num_i64_str(number: i64) -> String {
    let mut formatted_number = String::new();
    let mut count = 0;
    let number_convert = number.to_string();

    // Iterate through the characters of the number from right to left
    for c in number_convert.chars().rev() {
        if count == 3 {
            formatted_number.push(',');
            count = 0;
        }
        formatted_number.push(c);
        count += 1;
    }

    // Reverse the formatted number back to the original order
    formatted_number.chars().rev().collect()
}

// fn fmt_num_f64_str(number: f64) -> String {
//     let mut formatted_number = String::new();
//     let mut count = 0;
//     let number_convert = number.to_string();
//     let number_point_raw = number_convert.split(".");

//     let collection = number_point_raw.collect::<Vec<&str>>();
//     // dbg!(collection);
//     let number_convert = &collection[0];
//     let number_convert_last = &collection[1];

//     // Iterate through the characters of the number from right to left
//     for c in number_convert.chars().rev() {
//         if count == 3 {
//             formatted_number.push(',');
//             count = 0;
//         }
//         formatted_number.push(c);
//         count += 1;
//     }

//     // Reverse the formatted number back to the original order
//     let num_first: Vec<_> = formatted_number.chars().rev().collect();
//     let number_sum = num_first
//         .extend_from_slice(number_convert_last..map(|i| i).collect())
//         .unwrap();
//     number_sum
// }

fn fmt_num_f64_str(number: f64) -> String {
    let mut formatted_number = String::new();
    let mut count = 0;
    let number_convert = number.to_string();
    let mut number_point_raw = number_convert.split('.');

    let integer_part = number_point_raw.next().unwrap();
    let decimal_part = number_point_raw.next().unwrap_or("00");

    // Iterate through the characters of the integer part from right to left
    for c in integer_part.chars().rev() {
        if count == 3 {
            formatted_number.push(',');
            count = 0;
        }
        formatted_number.push(c);
        count += 1;
    }

    // Reverse the formatted number back to the original order
    formatted_number = formatted_number.chars().rev().collect();

    // Combine the integer and decimal parts
    formatted_number.push('.');
    formatted_number.push_str(decimal_part);

    formatted_number
}

fn fmt_num_isize_str(number: isize) -> String {
    let mut formatted_number = String::new();
    let mut count = 0;
    let number_convert = number.to_string();

    // Iterate through the characters of the number from right to left
    for c in number_convert.chars().rev() {
        if count == 3 {
            formatted_number.push(',');
            count = 0;
        }
        formatted_number.push(c);
        count += 1;
    }

    // Reverse the formatted number back to the original order
    formatted_number.chars().rev().collect()
}

fn fmt_num_str(number: &str) -> String {
    let mut formatted_number = String::new();
    let mut count = 0;

    // Iterate through the characters of the number from right to left
    for c in number.chars().rev() {
        if count == 3 {
            formatted_number.push(',');
            count = 0;
        }
        formatted_number.push(c);
        count += 1;
    }

    // Reverse the formatted number back to the original order
    formatted_number.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_string() {
        assert_eq!(fmt_num_i32_str(12345), "12,345");
    }

    #[test]
    fn i64_string() {
        assert_eq!(fmt_num_i64_str(12345), "12,345");
    }

    #[test]
    fn f64_string() {
        assert_eq!(fmt_num_f64_str(12345.00), "12,345.00");
        let test_f64 = 23.23;
        println!("{}", test_f64);
        assert_eq!(fmt_num_f64_str(23.23), "23.23");
        assert_eq!(fmt_num_f64_str(1234567.89), "1,234,567.89");
        assert_eq!(fmt_num_f64_str(1234567890.0), "1,234,567,890.00");
        assert_eq!(fmt_num_f64_str(1000.0), "1,000.00");
    }

    #[test]
    fn isize_string() {
        assert_eq!(fmt_num_isize_str(12345), "12,345");
    }

    #[test]
    fn str_string() {
        assert_eq!(fmt_num_str("12345"), "12,345");
    }
}
