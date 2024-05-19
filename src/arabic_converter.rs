use crate::{roman_converter, ROMAN_LETTERS};

fn get_arabic_number(letter: char) -> u16 {
    match letter {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

/// Converts a Roman number to an Arabic number
/// 
/// # Arguments
/// 
/// * `roman_number` - The Roman number to convert.
///
/// # Examples
///
/// ```
/// use italian_numbers::arabic_converter;
/// 
/// let result = arabic_converter(String::from("MD"));
/// assert_eq!(Ok(1500), result);
///
/// let result = arabic_converter(String::from("CDXC"));
/// assert_eq!(Ok(490), result);
///
/// let result = arabic_converter(String::from("MCMXC"));
/// assert_eq!(Ok(1990), result);
/// ```
pub fn arabic_converter(roman_number: String) -> Result<u16, &'static str> {
    let mut number: i16 = 0;

    for (i, letter) in roman_number.chars().enumerate() {
        if !ROMAN_LETTERS
            .iter()
            .any(|e| e.to_string() == letter.to_string())
        {
            return Err("invalid roman number");
        }

        let mut arabic_number = get_arabic_number(letter) as i16;

        if i + 1 < roman_number.len() {
            let next_arabic_number = get_arabic_number(roman_number.chars().nth(i + 1).unwrap());
            if (arabic_number as u16) < next_arabic_number {
                arabic_number = -arabic_number;
            }
        }

        number += arabic_number
    }

    let real_roman_number = roman_converter(number as u16)?;

    if real_roman_number != roman_number {
        return Err("invalid roman number");
    }

    Ok(number as u16)
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::arabic_converter;

    mod random {
        use super::*;

        #[test]
        fn test_mmmcmxcix() {
            assert_eq!(arabic_converter(String::from("MMMCMXCIX")), Ok(3999))
        }

        #[test]
        fn test_dcclxxxiv() {
            assert_eq!(arabic_converter(String::from("DCCLXXXIV")), Ok(784))
        }

        #[test]
        fn test_cxxiii() {
            assert_eq!(arabic_converter(String::from("CXXIII")), Ok(123))
        }

        #[test]
        fn test_mmcccxvii() {
            assert_eq!(arabic_converter(String::from("MMCCCXVII")), Ok(2317))
        }

        #[test]
        fn test_cmxlix() {
            assert_eq!(arabic_converter(String::from("CMXLIX")), Ok(949))
        }

        #[test]
        fn test_lxxix() {
            assert_eq!(arabic_converter(String::from("LXXIX")), Ok(79))
        }

        #[test]
        fn test_mcmxc() {
            assert_eq!(arabic_converter(String::from("MCMXC")), Ok(1990))
        }
    }

    mod tens {
        use super::*;

        #[test]
        fn test_i() {
            assert_eq!(arabic_converter(String::from("I")), Ok(1));
        }

        #[test]
        fn test_x() {
            assert_eq!(arabic_converter(String::from("X")), Ok(10));
        }

        #[test]
        fn test_xx() {
            assert_eq!(arabic_converter(String::from("XX")), Ok(20));
        }

        #[test]
        fn test_xxx() {
            assert_eq!(arabic_converter(String::from("XXX")), Ok(30));
        }

        #[test]
        fn test_xl() {
            assert_eq!(arabic_converter(String::from("XL")), Ok(40));
        }

        #[test]
        fn test_l() {
            assert_eq!(arabic_converter(String::from("L")), Ok(50));
        }

        #[test]
        fn test_lx() {
            assert_eq!(arabic_converter(String::from("LX")), Ok(60));
        }

        #[test]
        fn test_lxx() {
            assert_eq!(arabic_converter(String::from("LXX")), Ok(70));
        }

        #[test]
        fn test_lxxx() {
            assert_eq!(arabic_converter(String::from("LXXX")), Ok(80));
        }

        #[test]
        fn test_xc() {
            assert_eq!(arabic_converter(String::from("XC")), Ok(90));
        }

        #[test]
        fn test_c() {
            assert_eq!(arabic_converter(String::from("C")), Ok(100));
        }

        #[test]
        fn test_ccc() {
            assert_eq!(arabic_converter(String::from("CCC")), Ok(300));
        }

        #[test]
        fn test_d() {
            assert_eq!(arabic_converter(String::from("D")), Ok(500));
        }

        #[test]
        fn test_dcc() {
            assert_eq!(arabic_converter(String::from("DCC")), Ok(700));
        }

        #[test]
        fn test_dccxxx() {
            assert_eq!(arabic_converter(String::from("DCCXXX")), Ok(730));
        }

        #[test]
        fn test_cdxc() {
            assert_eq!(arabic_converter(String::from("CDXC")), Ok(490));
        }

        #[test]
        fn test_m() {
            assert_eq!(arabic_converter(String::from("M")), Ok(1000));
        }

        #[test]
        fn test_md() {
            assert_eq!(arabic_converter(String::from("MD")), Ok(1500));
        }

        #[test]
        fn test_mcmxc() {
            assert_eq!(arabic_converter(String::from("MCMXC")), Ok(1990));
        }
    }

    mod exceptions {
        use super::*;

        #[test]
        fn test_invalid_roman_number_1() {
            assert_eq!(arabic_converter(String::from("not a roman number")), Err("invalid roman number"));
        }

        #[test]
        fn test_invalid_roman_number_2() {
            assert_eq!(arabic_converter(String::from("XIXIX")), Err("invalid roman number"));
        }
    }
}
