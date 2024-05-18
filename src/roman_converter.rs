use crate::{ROMAN_HUNDREDS, ROMAN_TENS, ROMAN_THOUSANDS, ROMAN_UNITS};

/// Converts an Arabic number to a Roman number
///
/// # Examples
///
/// ```
/// use italian_numbers::roman_converter;
/// 
/// let result = roman_converter(1);
/// assert_eq!(Ok(String::from("I")), result);
///
/// let result = roman_converter(79);
/// assert_eq!(Ok(String::from("LXXIX")), result);
///
/// let result = roman_converter(2317);
/// assert_eq!(Ok(String::from("MMCCCXVII")), result);
/// ```
pub fn roman_converter(number: u16) -> Result<String, &'static str> {
    if number > 3999 {
        return Err("greater than 3999");
    }

    if number < 1 {
        return Err("lower than 1");
    }

    let unit = (number % 10) as usize;
    let ten = ((number / 10) % 10) as usize;
    let hundred = ((number / 100) % 10) as usize;
    let thousand = ((number / 1_000) % 10) as usize;

    Ok(format!(
        "{}{}{}{}",
        ROMAN_THOUSANDS[thousand], ROMAN_HUNDREDS[hundred], ROMAN_TENS[ten], ROMAN_UNITS[unit]
    ))
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::roman_converter;

    mod random {
        use super::*;

        #[test]
        fn test_3_999() {
            assert_eq!(roman_converter(3_999), Ok(String::from("MMMCMXCIX")));
        }

        #[test]
        fn test_784() {
            assert_eq!(roman_converter(784), Ok(String::from("DCCLXXXIV")));
        }

        #[test]
        fn test_123() {
            assert_eq!(roman_converter(123), Ok(String::from("CXXIII")));
        }

        #[test]
        fn test_2317() {
            assert_eq!(roman_converter(2317), Ok(String::from("MMCCCXVII")));
        }

        #[test]
        fn test_949() {
            assert_eq!(roman_converter(949), Ok(String::from("CMXLIX")));
        }

        #[test]
        fn test_79() {
            assert_eq!(roman_converter(79), Ok(String::from("LXXIX")));
        }
    }

    mod tens {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(roman_converter(1), Ok(String::from("I")));
        }

        #[test]
        fn test_10() {
            assert_eq!(roman_converter(10), Ok(String::from("X")));
        }

        #[test]
        fn test_20() {
            assert_eq!(roman_converter(20), Ok(String::from("XX")));
        }

        #[test]
        fn test_30() {
            assert_eq!(roman_converter(30), Ok(String::from("XXX")));
        }

        #[test]
        fn test_40() {
            assert_eq!(roman_converter(40), Ok(String::from("XL")));
        }

        #[test]
        fn test_50() {
            assert_eq!(roman_converter(50), Ok(String::from("L")));
        }

        #[test]
        fn test_60() {
            assert_eq!(roman_converter(60), Ok(String::from("LX")));
        }

        #[test]
        fn test_70() {
            assert_eq!(roman_converter(70), Ok(String::from("LXX")));
        }

        #[test]
        fn test_80() {
            assert_eq!(roman_converter(80), Ok(String::from("LXXX")));
        }

        #[test]
        fn test_90() {
            assert_eq!(roman_converter(90), Ok(String::from("XC")));
        }

        #[test]
        fn test_100() {
            assert_eq!(roman_converter(100), Ok(String::from("C")));
        }

        #[test]
        fn test_300() {
            assert_eq!(roman_converter(300), Ok(String::from("CCC")));
        }

        #[test]
        fn test_500() {
            assert_eq!(roman_converter(500), Ok(String::from("D")));
        }

        #[test]
        fn test_700() {
            assert_eq!(roman_converter(700), Ok(String::from("DCC")));
        }

        #[test]
        fn test_730() {
            assert_eq!(roman_converter(730), Ok(String::from("DCCXXX")));
        }

        #[test]
        fn test_490() {
            assert_eq!(roman_converter(490), Ok(String::from("CDXC")));
        }

        #[test]
        fn test_1_000() {
            assert_eq!(roman_converter(1_000), Ok(String::from("M")));
        }

        #[test]
        fn test_1_500() {
            assert_eq!(roman_converter(1_500), Ok(String::from("MD")));
        }

        #[test]
        fn test_1_990() {
            assert_eq!(roman_converter(1_990), Ok(String::from("MCMXC")));
        }
    }

    mod exceptions {
        use super::*;

        #[test]
        fn test_greater_than_3_999() {
            assert_eq!(roman_converter(4000), Err("greater than 3999"))
        }

        #[test]
        fn test_lower_than_1() {
            assert_eq!(roman_converter(0), Err("lower than 1"))
        }
    }
}
