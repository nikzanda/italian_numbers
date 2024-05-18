use crate::{cardinal_converter, ZERO_TEN_ORDINALS};
use regex::{Captures, Regex};

pub struct Options {
    female: bool,
    plural: bool,
}

impl Options {
    pub fn new(female: bool, plural: bool) -> Options {
        Options { female, plural }
    }
}

fn converter(number: u64) -> Result<String, &'static str> {
    if number < 11 {
        return Ok(ZERO_TEN_ORDINALS[number as usize].to_string());
    }

    let mut word = cardinal_converter(number as i64)?;

    word = word.replace(" e ", " ");
    word = Regex::new(r"([io])\s([ou])")
        .unwrap()
        .replace_all(&word, |caps: &Captures| {
            if &caps[2] == "o" {
                return caps[1].to_string();
            }
            if &caps[2] == "u" {
                return caps[2].to_string();
            }
            "".to_string()
        })
        .to_string();
    word = word.replace("un ", "");
    word = Regex::new(r"ouno$")
        .unwrap()
        .replace_all(&word, "uno")
        .to_string();
    word = Regex::new(r"ootto$")
        .unwrap()
        .replace_all(&word, "otto")
        .to_string();
    word = word.replace(' ', "");

    let last_digit = number % 10;

    if number % 100 != 13 && last_digit == 3 {
        word.pop();
        word.push('e');
        return Ok(format!("{}esimo", word));
    }
    if number % 100 != 16 && last_digit == 6 {
        return Ok(format!("{}esimo", word));
    }
    if number % 100 == 10 {
        return Ok(word.replace("dieci", "decimo"));
    }
    if word.ends_with("mila") {
        return Ok(word.replace("mila", "millesimo"));
    }

    word.pop();
    Ok(format!("{}esimo", word))
}

/// Converts a number to an italian word representation (ordinal number)
///
/// # Examples
///
/// ```
/// use italian_numbers::{ordinal_converter, Options};
///
/// /* Options None */
/// let result = ordinal_converter(1, None);
/// assert_eq!(Ok(String::from("primo")), result);
///
/// let result = ordinal_converter(10, None);
/// assert_eq!(Ok(String::from("decimo")), result);
///
/// let result = ordinal_converter(63, None);
/// assert_eq!(Ok(String::from("sessantatreesimo")), result);
///
/// /* female true */
/// let result = ordinal_converter(1, Some(Options::new(true, false)));
/// assert_eq!(Ok(String::from("prima")), result);
///
/// let result = ordinal_converter(15, Some(Options::new(true, false)));
/// assert_eq!(Ok(String::from("quindicesima")), result);
///
/// let result = ordinal_converter(109, Some(Options::new(true, false)));
/// assert_eq!(Ok(String::from("centonovesima")), result);
///
/// /* plural true */
/// let result = ordinal_converter(1, Some(Options::new(false, true)));
/// assert_eq!(Ok(String::from("primi")), result);
///
/// let result = ordinal_converter(18, Some(Options::new(false, true)));
/// assert_eq!(Ok(String::from("diciottesimi")), result);
///
/// let result = ordinal_converter(89, Some(Options::new(false, true)));
/// assert_eq!(Ok(String::from("ottantanovesimi")), result);
///
/// /* female and plural true */
/// let result = ordinal_converter(1, Some(Options::new(true, true)));
/// assert_eq!(Ok(String::from("prime")), result);
///
/// let result = ordinal_converter(70, Some(Options::new(true, true)));
/// assert_eq!(Ok(String::from("settantesime")), result);
///
/// let result = ordinal_converter(110, Some(Options::new(true, true)));
/// assert_eq!(Ok(String::from("centodecime")), result);
/// ```
pub fn ordinal_converter(number: u64, options: Option<Options>) -> Result<String, &'static str> {
    let mut result = converter(number)?;

    let female = options.as_ref().map_or(false, |o| o.female);
    let plural = options.as_ref().map_or(false, |o| o.plural);

    if female && plural {
        result.pop();
        result.push('e');
        return Ok(result);
    }

    if female {
        result.pop();
        result.push('a');
        return Ok(result);
    }

    if plural {
        result.pop();
        result.push('i');
        return Ok(result);
    }

    Ok(result)
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::ordinal_converter;

    mod first_thirty_numbers {
        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(ordinal_converter(1, None), Ok(String::from("primo")))
        }

        #[test]
        fn test_2() {
            assert_eq!(ordinal_converter(2, None), Ok(String::from("secondo")))
        }

        #[test]
        fn test_3() {
            assert_eq!(ordinal_converter(3, None), Ok(String::from("terzo")))
        }

        #[test]
        fn test_4() {
            assert_eq!(ordinal_converter(4, None), Ok(String::from("quarto")))
        }

        #[test]
        fn test_5() {
            assert_eq!(ordinal_converter(5, None), Ok(String::from("quinto")))
        }

        #[test]
        fn test_6() {
            assert_eq!(ordinal_converter(6, None), Ok(String::from("sesto")))
        }

        #[test]
        fn test_7() {
            assert_eq!(ordinal_converter(7, None), Ok(String::from("settimo")))
        }

        #[test]
        fn test_8() {
            assert_eq!(ordinal_converter(8, None), Ok(String::from("ottavo")))
        }

        #[test]
        fn test_9() {
            assert_eq!(ordinal_converter(9, None), Ok(String::from("nono")))
        }

        #[test]
        fn test_10() {
            assert_eq!(ordinal_converter(10, None), Ok(String::from("decimo")))
        }

        #[test]
        fn test_11() {
            assert_eq!(ordinal_converter(11, None), Ok(String::from("undicesimo")))
        }

        #[test]
        fn test_12() {
            assert_eq!(ordinal_converter(12, None), Ok(String::from("dodicesimo")))
        }

        #[test]
        fn test_13() {
            assert_eq!(ordinal_converter(13, None), Ok(String::from("tredicesimo")))
        }

        #[test]
        fn test_14() {
            assert_eq!(
                ordinal_converter(14, None),
                Ok(String::from("quattordicesimo"))
            )
        }

        #[test]
        fn test_15() {
            assert_eq!(
                ordinal_converter(15, None),
                Ok(String::from("quindicesimo"))
            )
        }

        #[test]
        fn test_16() {
            assert_eq!(ordinal_converter(16, None), Ok(String::from("sedicesimo")))
        }

        #[test]
        fn test_17() {
            assert_eq!(
                ordinal_converter(17, None),
                Ok(String::from("diciassettesimo"))
            )
        }

        #[test]
        fn test_18() {
            assert_eq!(
                ordinal_converter(18, None),
                Ok(String::from("diciottesimo"))
            )
        }

        #[test]
        fn test_19() {
            assert_eq!(
                ordinal_converter(19, None),
                Ok(String::from("diciannovesimo"))
            )
        }

        #[test]
        fn test_20() {
            assert_eq!(ordinal_converter(20, None), Ok(String::from("ventesimo")))
        }

        #[test]
        fn test_21() {
            assert_eq!(ordinal_converter(21, None), Ok(String::from("ventunesimo")))
        }

        #[test]
        fn test_22() {
            assert_eq!(
                ordinal_converter(22, None),
                Ok(String::from("ventiduesimo"))
            )
        }

        #[test]
        fn test_23() {
            assert_eq!(
                ordinal_converter(23, None),
                Ok(String::from("ventitreesimo"))
            )
        }

        #[test]
        fn test_24() {
            assert_eq!(
                ordinal_converter(24, None),
                Ok(String::from("ventiquattresimo"))
            )
        }

        #[test]
        fn test_25() {
            assert_eq!(
                ordinal_converter(25, None),
                Ok(String::from("venticinquesimo"))
            )
        }

        #[test]
        fn test_26() {
            assert_eq!(
                ordinal_converter(26, None),
                Ok(String::from("ventiseiesimo"))
            )
        }

        #[test]
        fn test_27() {
            assert_eq!(
                ordinal_converter(27, None),
                Ok(String::from("ventisettesimo"))
            )
        }

        #[test]
        fn test_28() {
            assert_eq!(
                ordinal_converter(28, None),
                Ok(String::from("ventottesimo"))
            )
        }

        #[test]
        fn test_29() {
            assert_eq!(
                ordinal_converter(29, None),
                Ok(String::from("ventinovesimo"))
            )
        }
    }

    mod first_thirty_female_numbers {
        use crate::ordinal_converter::Options;

        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                ordinal_converter(1, Some(Options::new(true, false))),
                Ok(String::from("prima"))
            )
        }

        #[test]
        fn test_2() {
            assert_eq!(
                ordinal_converter(2, Some(Options::new(true, false))),
                Ok(String::from("seconda"))
            )
        }

        #[test]
        fn test_3() {
            assert_eq!(
                ordinal_converter(3, Some(Options::new(true, false))),
                Ok(String::from("terza"))
            )
        }

        #[test]
        fn test_4() {
            assert_eq!(
                ordinal_converter(4, Some(Options::new(true, false))),
                Ok(String::from("quarta"))
            )
        }

        #[test]
        fn test_5() {
            assert_eq!(
                ordinal_converter(5, Some(Options::new(true, false))),
                Ok(String::from("quinta"))
            )
        }

        #[test]
        fn test_6() {
            assert_eq!(
                ordinal_converter(6, Some(Options::new(true, false))),
                Ok(String::from("sesta"))
            )
        }

        #[test]
        fn test_7() {
            assert_eq!(
                ordinal_converter(7, Some(Options::new(true, false))),
                Ok(String::from("settima"))
            )
        }

        #[test]
        fn test_8() {
            assert_eq!(
                ordinal_converter(8, Some(Options::new(true, false))),
                Ok(String::from("ottava"))
            )
        }

        #[test]
        fn test_9() {
            assert_eq!(
                ordinal_converter(9, Some(Options::new(true, false))),
                Ok(String::from("nona"))
            )
        }

        #[test]
        fn test_10() {
            assert_eq!(
                ordinal_converter(10, Some(Options::new(true, false))),
                Ok(String::from("decima"))
            )
        }

        #[test]
        fn test_11() {
            assert_eq!(
                ordinal_converter(11, Some(Options::new(true, false))),
                Ok(String::from("undicesima"))
            )
        }

        #[test]
        fn test_12() {
            assert_eq!(
                ordinal_converter(12, Some(Options::new(true, false))),
                Ok(String::from("dodicesima"))
            )
        }

        #[test]
        fn test_13() {
            assert_eq!(
                ordinal_converter(13, Some(Options::new(true, false))),
                Ok(String::from("tredicesima"))
            )
        }

        #[test]
        fn test_14() {
            assert_eq!(
                ordinal_converter(14, Some(Options::new(true, false))),
                Ok(String::from("quattordicesima"))
            )
        }

        #[test]
        fn test_15() {
            assert_eq!(
                ordinal_converter(15, Some(Options::new(true, false))),
                Ok(String::from("quindicesima"))
            )
        }

        #[test]
        fn test_16() {
            assert_eq!(
                ordinal_converter(16, Some(Options::new(true, false))),
                Ok(String::from("sedicesima"))
            )
        }

        #[test]
        fn test_17() {
            assert_eq!(
                ordinal_converter(17, Some(Options::new(true, false))),
                Ok(String::from("diciassettesima"))
            )
        }

        #[test]
        fn test_18() {
            assert_eq!(
                ordinal_converter(18, Some(Options::new(true, false))),
                Ok(String::from("diciottesima"))
            )
        }

        #[test]
        fn test_19() {
            assert_eq!(
                ordinal_converter(19, Some(Options::new(true, false))),
                Ok(String::from("diciannovesima"))
            )
        }

        #[test]
        fn test_20() {
            assert_eq!(
                ordinal_converter(20, Some(Options::new(true, false))),
                Ok(String::from("ventesima"))
            )
        }

        #[test]
        fn test_21() {
            assert_eq!(
                ordinal_converter(21, Some(Options::new(true, false))),
                Ok(String::from("ventunesima"))
            )
        }

        #[test]
        fn test_22() {
            assert_eq!(
                ordinal_converter(22, Some(Options::new(true, false))),
                Ok(String::from("ventiduesima"))
            )
        }

        #[test]
        fn test_23() {
            assert_eq!(
                ordinal_converter(23, Some(Options::new(true, false))),
                Ok(String::from("ventitreesima"))
            )
        }

        #[test]
        fn test_24() {
            assert_eq!(
                ordinal_converter(24, Some(Options::new(true, false))),
                Ok(String::from("ventiquattresima"))
            )
        }

        #[test]
        fn test_25() {
            assert_eq!(
                ordinal_converter(25, Some(Options::new(true, false))),
                Ok(String::from("venticinquesima"))
            )
        }

        #[test]
        fn test_26() {
            assert_eq!(
                ordinal_converter(26, Some(Options::new(true, false))),
                Ok(String::from("ventiseiesima"))
            )
        }

        #[test]
        fn test_27() {
            assert_eq!(
                ordinal_converter(27, Some(Options::new(true, false))),
                Ok(String::from("ventisettesima"))
            )
        }

        #[test]
        fn test_28() {
            assert_eq!(
                ordinal_converter(28, Some(Options::new(true, false))),
                Ok(String::from("ventottesima"))
            )
        }

        #[test]
        fn test_29() {
            assert_eq!(
                ordinal_converter(29, Some(Options::new(true, false))),
                Ok(String::from("ventinovesima"))
            )
        }
    }

    mod first_thirty_plural_numbers {
        use crate::ordinal_converter::Options;

        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                ordinal_converter(1, Some(Options::new(false, true))),
                Ok(String::from("primi"))
            )
        }

        #[test]
        fn test_2() {
            assert_eq!(
                ordinal_converter(2, Some(Options::new(false, true))),
                Ok(String::from("secondi"))
            )
        }

        #[test]
        fn test_3() {
            assert_eq!(
                ordinal_converter(3, Some(Options::new(false, true))),
                Ok(String::from("terzi"))
            )
        }

        #[test]
        fn test_4() {
            assert_eq!(
                ordinal_converter(4, Some(Options::new(false, true))),
                Ok(String::from("quarti"))
            )
        }

        #[test]
        fn test_5() {
            assert_eq!(
                ordinal_converter(5, Some(Options::new(false, true))),
                Ok(String::from("quinti"))
            )
        }

        #[test]
        fn test_6() {
            assert_eq!(
                ordinal_converter(6, Some(Options::new(false, true))),
                Ok(String::from("sesti"))
            )
        }

        #[test]
        fn test_7() {
            assert_eq!(
                ordinal_converter(7, Some(Options::new(false, true))),
                Ok(String::from("settimi"))
            )
        }

        #[test]
        fn test_8() {
            assert_eq!(
                ordinal_converter(8, Some(Options::new(false, true))),
                Ok(String::from("ottavi"))
            )
        }

        #[test]
        fn test_9() {
            assert_eq!(
                ordinal_converter(9, Some(Options::new(false, true))),
                Ok(String::from("noni"))
            )
        }

        #[test]
        fn test_10() {
            assert_eq!(
                ordinal_converter(10, Some(Options::new(false, true))),
                Ok(String::from("decimi"))
            )
        }

        #[test]
        fn test_11() {
            assert_eq!(
                ordinal_converter(11, Some(Options::new(false, true))),
                Ok(String::from("undicesimi"))
            )
        }

        #[test]
        fn test_12() {
            assert_eq!(
                ordinal_converter(12, Some(Options::new(false, true))),
                Ok(String::from("dodicesimi"))
            )
        }

        #[test]
        fn test_13() {
            assert_eq!(
                ordinal_converter(13, Some(Options::new(false, true))),
                Ok(String::from("tredicesimi"))
            )
        }

        #[test]
        fn test_14() {
            assert_eq!(
                ordinal_converter(14, Some(Options::new(false, true))),
                Ok(String::from("quattordicesimi"))
            )
        }

        #[test]
        fn test_15() {
            assert_eq!(
                ordinal_converter(15, Some(Options::new(false, true))),
                Ok(String::from("quindicesimi"))
            )
        }

        #[test]
        fn test_16() {
            assert_eq!(
                ordinal_converter(16, Some(Options::new(false, true))),
                Ok(String::from("sedicesimi"))
            )
        }

        #[test]
        fn test_17() {
            assert_eq!(
                ordinal_converter(17, Some(Options::new(false, true))),
                Ok(String::from("diciassettesimi"))
            )
        }

        #[test]
        fn test_18() {
            assert_eq!(
                ordinal_converter(18, Some(Options::new(false, true))),
                Ok(String::from("diciottesimi"))
            )
        }

        #[test]
        fn test_19() {
            assert_eq!(
                ordinal_converter(19, Some(Options::new(false, true))),
                Ok(String::from("diciannovesimi"))
            )
        }

        #[test]
        fn test_20() {
            assert_eq!(
                ordinal_converter(20, Some(Options::new(false, true))),
                Ok(String::from("ventesimi"))
            )
        }

        #[test]
        fn test_21() {
            assert_eq!(
                ordinal_converter(21, Some(Options::new(false, true))),
                Ok(String::from("ventunesimi"))
            )
        }

        #[test]
        fn test_22() {
            assert_eq!(
                ordinal_converter(22, Some(Options::new(false, true))),
                Ok(String::from("ventiduesimi"))
            )
        }

        #[test]
        fn test_23() {
            assert_eq!(
                ordinal_converter(23, Some(Options::new(false, true))),
                Ok(String::from("ventitreesimi"))
            )
        }

        #[test]
        fn test_24() {
            assert_eq!(
                ordinal_converter(24, Some(Options::new(false, true))),
                Ok(String::from("ventiquattresimi"))
            )
        }

        #[test]
        fn test_25() {
            assert_eq!(
                ordinal_converter(25, Some(Options::new(false, true))),
                Ok(String::from("venticinquesimi"))
            )
        }

        #[test]
        fn test_26() {
            assert_eq!(
                ordinal_converter(26, Some(Options::new(false, true))),
                Ok(String::from("ventiseiesimi"))
            )
        }

        #[test]
        fn test_27() {
            assert_eq!(
                ordinal_converter(27, Some(Options::new(false, true))),
                Ok(String::from("ventisettesimi"))
            )
        }

        #[test]
        fn test_28() {
            assert_eq!(
                ordinal_converter(28, Some(Options::new(false, true))),
                Ok(String::from("ventottesimi"))
            )
        }

        #[test]
        fn test_29() {
            assert_eq!(
                ordinal_converter(29, Some(Options::new(false, true))),
                Ok(String::from("ventinovesimi"))
            )
        }
    }

    mod first_thirty_plural_female_numbers {
        use crate::ordinal_converter::Options;

        use super::*;

        #[test]
        fn test_1() {
            assert_eq!(
                ordinal_converter(1, Some(Options::new(true, true))),
                Ok(String::from("prime"))
            )
        }

        #[test]
        fn test_2() {
            assert_eq!(
                ordinal_converter(2, Some(Options::new(true, true))),
                Ok(String::from("seconde"))
            )
        }

        #[test]
        fn test_3() {
            assert_eq!(
                ordinal_converter(3, Some(Options::new(true, true))),
                Ok(String::from("terze"))
            )
        }

        #[test]
        fn test_4() {
            assert_eq!(
                ordinal_converter(4, Some(Options::new(true, true))),
                Ok(String::from("quarte"))
            )
        }

        #[test]
        fn test_5() {
            assert_eq!(
                ordinal_converter(5, Some(Options::new(true, true))),
                Ok(String::from("quinte"))
            )
        }

        #[test]
        fn test_6() {
            assert_eq!(
                ordinal_converter(6, Some(Options::new(true, true))),
                Ok(String::from("seste"))
            )
        }

        #[test]
        fn test_7() {
            assert_eq!(
                ordinal_converter(7, Some(Options::new(true, true))),
                Ok(String::from("settime"))
            )
        }

        #[test]
        fn test_8() {
            assert_eq!(
                ordinal_converter(8, Some(Options::new(true, true))),
                Ok(String::from("ottave"))
            )
        }

        #[test]
        fn test_9() {
            assert_eq!(
                ordinal_converter(9, Some(Options::new(true, true))),
                Ok(String::from("none"))
            )
        }

        #[test]
        fn test_10() {
            assert_eq!(
                ordinal_converter(10, Some(Options::new(true, true))),
                Ok(String::from("decime"))
            )
        }

        #[test]
        fn test_11() {
            assert_eq!(
                ordinal_converter(11, Some(Options::new(true, true))),
                Ok(String::from("undicesime"))
            )
        }

        #[test]
        fn test_12() {
            assert_eq!(
                ordinal_converter(12, Some(Options::new(true, true))),
                Ok(String::from("dodicesime"))
            )
        }

        #[test]
        fn test_13() {
            assert_eq!(
                ordinal_converter(13, Some(Options::new(true, true))),
                Ok(String::from("tredicesime"))
            )
        }

        #[test]
        fn test_14() {
            assert_eq!(
                ordinal_converter(14, Some(Options::new(true, true))),
                Ok(String::from("quattordicesime"))
            )
        }

        #[test]
        fn test_15() {
            assert_eq!(
                ordinal_converter(15, Some(Options::new(true, true))),
                Ok(String::from("quindicesime"))
            )
        }

        #[test]
        fn test_16() {
            assert_eq!(
                ordinal_converter(16, Some(Options::new(true, true))),
                Ok(String::from("sedicesime"))
            )
        }

        #[test]
        fn test_17() {
            assert_eq!(
                ordinal_converter(17, Some(Options::new(true, true))),
                Ok(String::from("diciassettesime"))
            )
        }

        #[test]
        fn test_18() {
            assert_eq!(
                ordinal_converter(18, Some(Options::new(true, true))),
                Ok(String::from("diciottesime"))
            )
        }

        #[test]
        fn test_19() {
            assert_eq!(
                ordinal_converter(19, Some(Options::new(true, true))),
                Ok(String::from("diciannovesime"))
            )
        }

        #[test]
        fn test_20() {
            assert_eq!(
                ordinal_converter(20, Some(Options::new(true, true))),
                Ok(String::from("ventesime"))
            )
        }

        #[test]
        fn test_21() {
            assert_eq!(
                ordinal_converter(21, Some(Options::new(true, true))),
                Ok(String::from("ventunesime"))
            )
        }

        #[test]
        fn test_22() {
            assert_eq!(
                ordinal_converter(22, Some(Options::new(true, true))),
                Ok(String::from("ventiduesime"))
            )
        }

        #[test]
        fn test_23() {
            assert_eq!(
                ordinal_converter(23, Some(Options::new(true, true))),
                Ok(String::from("ventitreesime"))
            )
        }

        #[test]
        fn test_24() {
            assert_eq!(
                ordinal_converter(24, Some(Options::new(true, true))),
                Ok(String::from("ventiquattresime"))
            )
        }

        #[test]
        fn test_25() {
            assert_eq!(
                ordinal_converter(25, Some(Options::new(true, true))),
                Ok(String::from("venticinquesime"))
            )
        }

        #[test]
        fn test_26() {
            assert_eq!(
                ordinal_converter(26, Some(Options::new(true, true))),
                Ok(String::from("ventiseiesime"))
            )
        }

        #[test]
        fn test_27() {
            assert_eq!(
                ordinal_converter(27, Some(Options::new(true, true))),
                Ok(String::from("ventisettesime"))
            )
        }

        #[test]
        fn test_28() {
            assert_eq!(
                ordinal_converter(28, Some(Options::new(true, true))),
                Ok(String::from("ventottesime"))
            )
        }

        #[test]
        fn test_29() {
            assert_eq!(
                ordinal_converter(29, Some(Options::new(true, true))),
                Ok(String::from("ventinovesime"))
            )
        }
    }

    mod exceptions {
        use super::*;

        #[test]
        fn test_greater_than_999_999_999_999() {
            assert_eq!(
                ordinal_converter(999_999_999_999 + 1, None),
                Err("greater than 999.999.999.999")
            );
        }
    }

    mod min_max {
        use super::*;

        #[test]
        fn test_999_999_999_999() {
            assert_eq!(ordinal_converter(999_999_999_999, None), Ok(String::from("novecentonovantanovemiliardinovecentonovantanovemilioninovecentonovantanovemilanovecentonovantanovesimo")));
        }

        #[test]
        fn test_0() {
            assert_eq!(ordinal_converter(0, None), Ok(String::from("zeresimo")));
        }
    }

    mod hundreds {
        use super::*;

        #[test]
        fn test_101() {
            assert_eq!(
                ordinal_converter(101, None),
                Ok(String::from("centunesimo"))
            )
        }

        #[test]
        fn test_108() {
            assert_eq!(
                ordinal_converter(108, None),
                Ok(String::from("centottesimo"))
            )
        }

        #[test]
        fn test_110() {
            assert_eq!(
                ordinal_converter(110, None),
                Ok(String::from("centodecimo"))
            )
        }

        #[test]
        fn test_116() {
            assert_eq!(
                ordinal_converter(116, None),
                Ok(String::from("centosedicesimo"))
            )
        }
    }

    mod random {
        use super::*;

        #[test]
        fn test_801_100() {
            assert_eq!(
                ordinal_converter(801_100, None),
                Ok(String::from("ottocentounomilacentesimo"))
            );
        }

        #[test]
        fn test_108_416() {
            assert_eq!(
                ordinal_converter(108_416, None),
                Ok(String::from("centoottomilaquattrocentosedicesimo"))
            );
        }

        #[test]
        fn test_1_000() {
            assert_eq!(
                ordinal_converter(1_000, None),
                Ok(String::from("millesimo"))
            );
        }

        #[test]
        fn test_1_110() {
            assert_eq!(
                ordinal_converter(1_110, None),
                Ok(String::from("millecentodecimo"))
            );
        }

        #[test]
        fn test_1_000_000() {
            assert_eq!(
                ordinal_converter(1_000_000, None),
                Ok(String::from("milionesimo"))
            );
        }

        #[test]
        fn test_1_000_000_000() {
            assert_eq!(
                ordinal_converter(1_000_000_000, None),
                Ok(String::from("miliardesimo"))
            );
        }

        #[test]
        fn test_1_000_001() {
            assert_eq!(
                ordinal_converter(1_000_001, None),
                Ok(String::from("milioneunesimo"))
            );
        }

        #[test]
        fn test_1_000_000_001() {
            assert_eq!(
                ordinal_converter(1_000_000_001, None),
                Ok(String::from("miliardunesimo"))
            );
        }

        #[test]
        fn test_1_000_000_008() {
            assert_eq!(
                ordinal_converter(1_000_000_008, None),
                Ok(String::from("miliardottesimo"))
            );
        }

        #[test]
        fn test_2_000() {
            assert_eq!(
                ordinal_converter(2_000, None),
                Ok(String::from("duemillesimo"))
            );
        }

        #[test]
        fn test_2_000_000_001() {
            assert_eq!(
                ordinal_converter(2_000_000_001, None),
                Ok(String::from("duemiliardunesimo"))
            );
        }

        #[test]
        fn test_456_799_123() {
            assert_eq!(
                ordinal_converter(456_799_123, None),
                Ok(String::from(
                    "quattrocentocinquantaseimilionisettecentonovantanovemilacentoventitreesimo"
                ))
            );
        }

        #[test]
        fn test_103() {
            assert_eq!(
                ordinal_converter(103, None),
                Ok(String::from("centotreesimo"))
            );
        }
    }
}
