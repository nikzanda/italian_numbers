use crate::{AND, BILLIONS, HUNDRED, MILLIONS, TENS, THOUSANDS, ZERO_NINETEEN};

fn tens_converter(number: u64) -> String {
    if number < 20 {
        return ZERO_NINETEEN[number as usize].to_string();
    }

    let first_digit = (number / 10) as usize - 2;
    let second_digit = (number % 10) as u8;
    let mut word = TENS[first_digit].to_string();

    if second_digit == 1 || second_digit == 8 {
        word.pop();
    }

    if second_digit != 0 {
        word.push_str(ZERO_NINETEEN[second_digit as usize]);
    }

    word
}

fn hundreds_converter(number: u64) -> String {
    let first_digit = (number / 100) as usize;
    if first_digit == 1 {
        return HUNDRED.to_string();
    }

    format!("{}{}", ZERO_NINETEEN[first_digit], HUNDRED)
}

fn word_calculator(number: u64) -> String {
    match number {
        // tens
        0..=99 => tens_converter(number),

        // hundreds
        100..=999 => {
            if number % 100 == 0 {
                return hundreds_converter(number);
            }

            if number % 100 > 79 && number % 100 < 90 {
                let word = hundreds_converter(number);
                return format!(
                    "{}{}",
                    &word[0..word.len() - 1],
                    word_calculator(number % 100)
                );
            }

            format!(
                "{}{}",
                hundreds_converter(number),
                word_calculator(number % 100)
            )
        }

        // thousands
        1_000..=999_999 => {
            if number == 1_000 {
                return THOUSANDS[0].to_string();
            }

            let first_digits = number / 1_000;
            if first_digits == 1 {
                return format!("{}{}", THOUSANDS[0], word_calculator(number % 1_000));
            }

            if number % 1_000 == 0 {
                return format!("{}{}", word_calculator(first_digits), THOUSANDS[1]);
            }

            format!(
                "{}{}{}",
                word_calculator(first_digits),
                THOUSANDS[1],
                word_calculator(number % 1_000)
            )
        }

        // millions
        1_000_000..=999_999_999 => {
            if number == 1_000_000 {
                return MILLIONS[0].to_string();
            }

            let first_digits = number / 1_000_000;
            if first_digits == 1 {
                return format!(
                    "{}{}{}",
                    MILLIONS[0],
                    AND,
                    word_calculator(number % 1_000_000)
                );
            }

            if number % 1_000_000 == 0 {
                return format!("{}{}", word_calculator(first_digits), MILLIONS[1]);
            }

            format!(
                "{}{}{}{}",
                word_calculator(first_digits),
                MILLIONS[1],
                AND,
                word_calculator(number % 1_000_000)
            )
        }

        // billions
        1_000_000_000..=999_999_999_999 => {
            if number == 1_000_000_000 {
                return BILLIONS[0].to_string();
            }

            let first_digits = number / 1_000_000_000;
            if first_digits == 1 {
                return format!(
                    "{}{}{}",
                    BILLIONS[0],
                    AND,
                    word_calculator(number % 1_000_000_000)
                );
            }

            if number % 1_000_000_000 == 0 {
                return format!("{}{}", word_calculator(first_digits), BILLIONS[1]);
            }

            format!(
                "{}{}{}{}",
                word_calculator(first_digits),
                BILLIONS[1],
                AND,
                word_calculator(number % 1_000_000_000)
            )
        }

        _ => "Numero non supportato".to_string(),
    }
}

fn replace_threes_occurrences(word: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    for (i, _c) in word.match_indices("tre ") {
        if i > 0 && word.chars().nth(i - 1) != Some(' ') {
            result.push_str(&word[last_end..i]);
            result.push_str("tré ");
            last_end = i + 4;
        }
    }

    result.push_str(&word[last_end..]);
    result
}

/// Converts a number to its Italian cardinal representation.
///
/// # Arguments
///
/// * `number` - The number to convert.
///
/// * `bool` - If true, the function will return number with decimals
///
/// # Examples
///
/// ```
/// use italian_numbers::cardinal_converter;
///
/// let result = cardinal_converter(1.0, false);
/// assert_eq!(Ok(String::from("uno")), result);
///
/// let result = cardinal_converter(90.0, false);
/// assert_eq!(Ok(String::from("novanta")), result);
///
/// let result = cardinal_converter(709.0, false);
/// assert_eq!(Ok(String::from("settecentonove")), result);
///
/// let result = cardinal_converter(1000.05, true);
/// assert_eq!(Ok(String::from("mille/05")), result);
///
/// let result = cardinal_converter(9.0, true);
/// assert_eq!(Ok(String::from("nove/00")), result);
///
/// let result = cardinal_converter(-1.0, false);
/// assert_eq!(Ok(String::from("meno uno")), result);
/// ```
pub fn cardinal_converter(number: f64, include_decimals: bool) -> Result<String, &'static str> {
    if number.is_infinite() {
        return Ok(String::from("infinito"));
    }
    if number > 999_999_999_999.99 {
        return Err("greater than 999.999.999.999");
    }
    if number < -999_999_999_999.99 {
        return Err("lower than 999.999.999.999");
    }

    let sign = if number < 0.0 { "meno " } else { "" };
    let abs_number = number.abs();

    let mut word = word_calculator(abs_number as u64);
    if word.ends_with("tre") && word != "tre" {
        word.pop();
        word.push('é');
    }

    let mut result = replace_threes_occurrences(&word);

    if include_decimals {
        let decimals = ((abs_number * 100.0).floor() % 100.0) as u64;
        result = format!("{}/{}", result, format!("{:02}", decimals));
    }

    Ok(format!("{}{}", sign, result))
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::cardinal_converter;

    mod one_and_eight {
        use super::*;

        #[test]
        fn test_21() {
            assert_eq!(cardinal_converter(21.0, false), Ok(String::from("ventuno")))
        }

        #[test]
        fn test_28() {
            assert_eq!(
                cardinal_converter(28.0, false),
                Ok(String::from("ventotto"))
            )
        }
    }

    mod exceptions {
        use super::*;

        #[test]
        fn test_greater_than_999_999_999_999() {
            assert_eq!(
                cardinal_converter(999_999_999_999.0 + 1.0, false),
                Err("greater than 999.999.999.999")
            )
        }
    }

    mod ones {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(cardinal_converter(1.0, false), Ok(String::from("uno")))
        }

        #[test]
        fn test_100() {
            assert_eq!(cardinal_converter(100.0, false), Ok(String::from("cento")))
        }

        #[test]
        fn test_1_000() {
            assert_eq!(
                cardinal_converter(1_000.0, false),
                Ok(String::from("mille"))
            )
        }

        #[test]
        fn test_1_000_000() {
            assert_eq!(
                cardinal_converter(1_000_000.0, false),
                Ok(String::from("un milione"))
            )
        }

        #[test]
        fn test_1_000_000_000() {
            assert_eq!(
                cardinal_converter(1_000_000_000.0, false),
                Ok(String::from("un miliardo"))
            )
        }
    }

    mod threes {
        use super::*;

        #[test]
        fn test_3() {
            assert_eq!(cardinal_converter(3.0, false), Ok(String::from("tre")))
        }

        #[test]
        fn test_33() {
            assert_eq!(
                cardinal_converter(33.0, false),
                Ok(String::from("trentatré"))
            )
        }

        #[test]
        fn test_333() {
            assert_eq!(
                cardinal_converter(333.0, false),
                Ok(String::from("trecentotrentatré"))
            )
        }

        #[test]
        fn test_3_333() {
            assert_eq!(
                cardinal_converter(3_333.0, false),
                Ok(String::from("tremilatrecentotrentatré"))
            )
        }

        #[test]
        fn test_3_000_000() {
            assert_eq!(
                cardinal_converter(3_000_000.0, false),
                Ok(String::from("tre milioni"))
            )
        }

        #[test]
        fn test_3_000_033() {
            assert_eq!(
                cardinal_converter(3_000_033.0, false),
                Ok(String::from("tre milioni e trentatré"))
            )
        }

        #[test]
        fn test_33_003_000() {
            assert_eq!(
                cardinal_converter(33_003_000.0, false),
                Ok(String::from("trentatré milioni e tremila"))
            )
        }

        #[test]
        fn test_3_033_000_000() {
            assert_eq!(
                cardinal_converter(3_033_000_000.0, false),
                Ok(String::from("tre miliardi e trentatré milioni"))
            )
        }

        #[test]
        fn test_3_003_000_000() {
            assert_eq!(
                cardinal_converter(3_003_000_000.0, false),
                Ok(String::from("tre miliardi e tre milioni"))
            )
        }

        #[test]
        fn test_3_000() {
            assert_eq!(
                cardinal_converter(3_000.0, false),
                Ok(String::from("tremila"))
            )
        }

        #[test]
        fn test_23_000() {
            assert_eq!(
                cardinal_converter(23_000.0, false),
                Ok(String::from("ventitremila"))
            )
        }

        #[test]
        fn test_103_103_103_103() {
            assert_eq!(
                cardinal_converter(103_103_103_103.0, false),
                Ok(String::from(
                    "centotré miliardi e centotré milioni e centotremilacentotré"
                ))
            )
        }
    }

    mod min_max {
        use std::f64::INFINITY;

        use super::*;

        #[test]
        fn test_999_999_999_999() {
            assert_eq!(cardinal_converter(999_999_999_999.0, false), Ok(String::from("novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove")))
        }

        #[test]
        fn test_negative_999_999_999_999() {
            assert_eq!(cardinal_converter(-999_999_999_999.0, false), Ok(String::from("meno novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove")))
        }

        #[test]
        fn test_0() {
            assert_eq!(cardinal_converter(0.0, false), Ok(String::from("zero")))
        }

        #[test]
        fn test_infinity() {
            assert_eq!(
                cardinal_converter(INFINITY, false),
                Ok(String::from("infinito"))
            )
        }
    }

    mod decimals {
        use super::*;

        #[test]
        fn test_9() {
            assert_eq!(cardinal_converter(9.0, true), Ok(String::from("nove/00")));
        }

        #[test]
        fn test_10_dot_45() {
            assert_eq!(
                cardinal_converter(10.45, true),
                Ok(String::from("dieci/45"))
            );
        }

        #[test]
        fn test_10_dot_00() {
            assert_eq!(
                cardinal_converter(10.00, true),
                Ok(String::from("dieci/00"))
            );
        }

        #[test]
        fn test_10_dot_06() {
            assert_eq!(
                cardinal_converter(10.06, true),
                Ok(String::from("dieci/06"))
            );
        }

        #[test]
        fn test_145_dot_6() {
            assert_eq!(
                cardinal_converter(145.6, true),
                Ok(String::from("centoquarantacinque/60"))
            );
        }

        #[test]
        fn test_3450_dot_0() {
            assert_eq!(
                cardinal_converter(3450.0, true),
                Ok(String::from("tremilaquattrocentocinquanta/00"))
            );
        }

        #[test]
        fn test_10000_dot_999() {
            assert_eq!(
                cardinal_converter(10000.999, true),
                Ok(String::from("diecimila/99"))
            );
        }

        #[test]
        fn test_10000_dot_99999999979() {
            assert_eq!(
                cardinal_converter(10000.99999999979, true),
                Ok(String::from("diecimila/99"))
            );
        }
    }

    mod random {
        use super::*;

        #[test]
        fn test_200() {
            assert_eq!(
                cardinal_converter(200.0, false),
                Ok(String::from("duecento"))
            )
        }

        #[test]
        fn test_27_347_687() {
            assert_eq!(
                cardinal_converter(27_347_687.0, false),
                Ok(String::from(
                    "ventisette milioni e trecentoquarantasettemilaseicentottantasette"
                ))
            )
        }

        #[test]
        fn test_200_000_000() {
            assert_eq!(
                cardinal_converter(200_000_000.0, false),
                Ok(String::from("duecento milioni"))
            )
        }

        #[test]
        fn test_12_341() {
            assert_eq!(
                cardinal_converter(12_341.0, false),
                Ok(String::from("dodicimilatrecentoquarantuno"))
            )
        }

        #[test]
        fn test_negative_34_564() {
            assert_eq!(
                cardinal_converter(-34_564.0, false),
                Ok(String::from(
                    "meno trentaquattromilacinquecentosessantaquattro"
                ))
            )
        }

        #[test]
        fn test_2_398_406() {
            assert_eq!(
                cardinal_converter(2_398_406.0, false),
                Ok(String::from(
                    "due milioni e trecentonovantottomilaquattrocentosei"
                ))
            )
        }

        #[test]
        fn test_9_654_367() {
            assert_eq!(
                cardinal_converter(9_654_367.0, false),
                Ok(String::from(
                    "nove milioni e seicentocinquantaquattromilatrecentosessantasette"
                ))
            )
        }

        #[test]
        fn test_100_100_100() {
            assert_eq!(
                cardinal_converter(100_100_100.0, false),
                Ok(String::from("cento milioni e centomilacento"))
            )
        }
    }
}
