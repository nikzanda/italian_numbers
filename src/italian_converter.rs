use regex::Regex;

use crate::{TENS, ZERO_TEN_ORDINALS};

fn get_unit(word: &str) -> Result<u8, &'static str> {
    match word {
        "un" => Ok(1),
        "uno" => Ok(1),
        "due" => Ok(2),
        "tre" => Ok(3),
        "tré" => Ok(3),
        "quattro" => Ok(4),
        "cinque" => Ok(5),
        "sei" => Ok(6),
        "sette" => Ok(7),
        "otto" => Ok(8),
        "nove" => Ok(9),
        "dieci" => Ok(10),
        "undici" => Ok(11),
        "dodici" => Ok(12),
        "tredici" => Ok(13),
        "quattordici" => Ok(14),
        "quindici" => Ok(15),
        "sedici" => Ok(16),
        "diciassette" => Ok(17),
        "diciotto" => Ok(18),
        "diciannove" => Ok(19),
        _ => Err("Not found"),
    }
}

fn get_ten(word: &str) -> Result<u8, &'static str> {
    match word {
        "venti" => Ok(20),
        "trenta" => Ok(30),
        "quaranta" => Ok(40),
        "cinquanta" => Ok(50),
        "sessanta" => Ok(60),
        "settanta" => Ok(70),
        "ottanta" => Ok(80),
        "novanta" => Ok(90),
        _ => Err("Not found"),
    }
}

fn tens_converter(word: &str) -> Result<u8, &'static str> {
    if let Ok(unit) = get_unit(word) {
        return Ok(unit);
    }

    let ten = TENS
        .iter()
        .find(|&ten| word.starts_with(&ten[..ten.len() - 1]))
        .ok_or("Not found")?;

    let result = get_ten(ten).unwrap();
    if word == *ten {
        return Ok(result);
    }

    let split_word = |separator| word.split(separator).collect::<Vec<&str>>();
    let mut unit = split_word(ten);
    if unit.len() < 2 {
        unit = split_word(&&ten[..ten.len() - 1]);
    }

    let unit_word = &unit.get(1).unwrap_or(&"").to_string();

    match get_unit(unit_word) {
        Ok(value) => Ok(result + value),
        Err(_) => Err("Not found"),
    }
}

fn hundreds_converter(word: &str) -> Result<u16, &'static str> {
    let mut split = word.split("cento");
    let hundred = split.next().unwrap_or("");
    let ten = split.next();

    if ten.is_none() {
        return tens_converter(hundred).map(|v| v as u16);
    }

    let mut result: u16;
    if hundred.is_empty() {
        result = 100;
    } else {
        match get_unit(hundred) {
            Ok(v) => result = (v as u16) * 100,
            Err(_) => return Err("Not found"),
        }
    }

    let real_ten = ten.unwrap_or("");
    if !real_ten.is_empty() {
        let tens_word = if real_ten.starts_with("tt") {
            format!("o{}", real_ten)
        } else {
            real_ten.to_string()
        };

        match tens_converter(&tens_word) {
            Ok(v) => {
                result += v as u16;
            }
            Err(_) => return Err("Not found"),
        }
    }

    Ok(result)
}

fn numbers_calculator(word: &str) -> Result<i64, &'static str> {
    // billions
    let mut index = word.find("miliard");
    if let Some(i) = index {
        let sub = &word[i + 8..];
        let mut rest = 0;
        if !sub.is_empty() {
            match numbers_calculator(sub) {
                Ok(v) => rest = v,
                Err(_) => return Err("not found"),
            }
        }

        return match hundreds_converter(&word[0..i]) {
            Ok(v) => Ok(v as i64 * 1_000_000_000 + rest),
            Err(_) => Err("not found"),
        };
    }

    // millions
    index = word.find("milion");
    if let Some(i) = index {
        let sub = &word[i + 7..];
        let mut rest = 0;
        if !sub.is_empty() {
            match numbers_calculator(sub) {
                Ok(v) => rest = v,
                Err(_) => return Err("not found"),
            }
        }

        return match hundreds_converter(&word[0..i]) {
            Ok(v) => Ok(v as i64 * 1_000_000 + rest),
            Err(_) => Err("not found"),
        };
    }

    // thounsands
    index = word.find("mila");
    if let Some(i) = index {
        let sub = &word[i + 4..];
        let mut rest = 0;
        if !sub.is_empty() {
            match numbers_calculator(sub) {
                Ok(v) => rest = v,
                Err(_) => return Err("not found"),
            }
        }

        return match hundreds_converter(&word[0..i]) {
            Ok(v) => Ok(v as i64 * 1_000 + rest),
            Err(_) => Err("not found"),
        };
    }

    index = word.find("mille");
    if let Some(i) = index {
        let sub = &word[i + 5..];
        if sub.is_empty() {
            return Ok(1_000);
        }

        return match hundreds_converter(&sub) {
            Ok(v) => Ok(v as i64 + 1_000),
            Err(_) => Err("not found"),
        };
    }

    hundreds_converter(&word).map(|v| v as i64)
}

fn get_number_from_ordinal(ordinal: &str) -> String {
    let mut cardinal = ordinal.to_string();
    let re_decim = Regex::new("decim\\w$").unwrap();
    let re_centesim = Regex::new("centesim\\w$").unwrap();
    let re_millesim_one = Regex::new("^millesim\\w").unwrap();
    let re_millesim_many = Regex::new("millesim\\w").unwrap();
    let re_milionesim_one = Regex::new("^milionesim\\w").unwrap();
    let re_milionesim_many = Regex::new("milionesim\\w").unwrap();
    let re_miliardesim_one = Regex::new("^miliardesim\\w").unwrap();
    let re_miliardesim_many = Regex::new("miliardesim\\w").unwrap();
    let re_ventesim = Regex::new("ventesim\\w").unwrap();
    let re_ntesim = Regex::new("ntesim\\w").unwrap();
    let re_unesim = Regex::new("unesim\\w").unwrap();
    let re_quattresim = Regex::new("quattresim\\w").unwrap();
    let re_ottesim = Regex::new("ottesim\\w").unwrap();
    let re_duesim = Regex::new("duesim\\w").unwrap();
    let re_cinquesim = Regex::new("cinquesim\\w").unwrap();
    let re_settesim = Regex::new("settesim\\w").unwrap();
    let re_novesim = Regex::new("novesim\\w").unwrap();
    let re_undicesim = Regex::new("undicesim\\w").unwrap();
    let re_dodicesim = Regex::new("dodicesim\\w").unwrap();
    let re_tredicesim = Regex::new("tredicesim\\w").unwrap();
    let re_quattordicesim = Regex::new("quattordicesim\\w").unwrap();
    let re_quindicesim = Regex::new("quindicesim\\w").unwrap();
    let re_sedicesim = Regex::new("sedicesim\\w").unwrap();
    let re_esim = Regex::new("esim\\w").unwrap();
    let re_milione = Regex::new("^milione").unwrap();
    let re_miliard = Regex::new("^miliard").unwrap();
    let re_centuno = Regex::new("centuno").unwrap();
    let re_centotto = Regex::new("centotto").unwrap();
    let re_miliarduno = Regex::new("miliarduno").unwrap();
    let re_miliardotto = Regex::new("miliardotto").unwrap();

    cardinal = re_decim.replace_all(&cardinal, "dieci").to_string();
    cardinal = re_centesim.replace_all(&cardinal, "cento").to_string();
    cardinal = re_millesim_one.replace_all(&cardinal, "mille").to_string();
    cardinal = re_millesim_many.replace_all(&cardinal, "mila").to_string();
    cardinal = re_milionesim_one
        .replace_all(&cardinal, "unmilione")
        .to_string();
    cardinal = re_milionesim_many
        .replace_all(&cardinal, "milioni")
        .to_string();
    cardinal = re_miliardesim_one
        .replace_all(&cardinal, "unmiliardo")
        .to_string();
    cardinal = re_miliardesim_many
        .replace_all(&cardinal, "miliardi")
        .to_string();
    cardinal = re_ventesim.replace_all(&cardinal, "venti").to_string();
    cardinal = re_ntesim.replace_all(&cardinal, "nta").to_string();
    cardinal = re_unesim.replace_all(&cardinal, "uno").to_string();
    cardinal = re_quattresim.replace_all(&cardinal, "quattro").to_string();
    cardinal = re_ottesim.replace_all(&cardinal, "otto").to_string();
    cardinal = re_duesim.replace_all(&cardinal, "due").to_string();
    cardinal = re_cinquesim.replace_all(&cardinal, "cinque").to_string();
    cardinal = re_settesim.replace_all(&cardinal, "sette").to_string();
    cardinal = re_novesim.replace_all(&cardinal, "nove").to_string();
    cardinal = re_undicesim.replace_all(&cardinal, "undici").to_string();
    cardinal = re_dodicesim.replace_all(&cardinal, "dodici").to_string();
    cardinal = re_tredicesim.replace_all(&cardinal, "tredici").to_string();
    cardinal = re_quattordicesim
        .replace_all(&cardinal, "quattordici")
        .to_string();
    cardinal = re_quindicesim
        .replace_all(&cardinal, "quindici")
        .to_string();
    cardinal = re_sedicesim.replace_all(&cardinal, "sedici").to_string();
    cardinal = re_esim.replace_all(&cardinal, "").to_string();
    cardinal = re_milione.replace_all(&cardinal, "unmilione").to_string();
    cardinal = re_miliard.replace_all(&cardinal, "unmiliardo").to_string();
    cardinal = re_centuno.replace_all(&cardinal, "centouno").to_string();
    cardinal = re_centotto.replace_all(&cardinal, "centootto").to_string();
    cardinal = re_miliarduno
        .replace_all(&cardinal, "miliardouno")
        .to_string();
    cardinal = re_miliardotto
        .replace_all(&cardinal, "miliardootto")
        .to_string();

    cardinal
}

/// Converts an Italian word representation to a number
/// 
/// # Arguments
/// 
/// * `word` - The Italian word to convert.
///
/// # Examples
///
/// ```
/// use italian_numbers::{italian_converter};
///
/// let result = italian_converter("uno".to_string());
/// assert_eq!(Ok(1), result);
///
/// let result = italian_converter("novantasette".to_string());
/// assert_eq!(Ok(97), result);
///
/// let result = italian_converter("un milione tredicimila".to_string());
/// assert_eq!(Ok(1013000), result);
///
/// let result = italian_converter("zeresimo".to_string());
/// assert_eq!(Ok(0), result);
///
/// let result = italian_converter("prima".to_string());
/// assert_eq!(Ok(1), result);
///
/// let result = italian_converter("quattrocentotredicesime".to_string());
/// assert_eq!(Ok(413), result);
/// ```
pub fn italian_converter(word: String) -> Result<i64, &'static str> {
    let mut escaped_word = word.to_lowercase();
    escaped_word = escaped_word.replace(" e ", "");
    escaped_word = escaped_word.replace(" ", "");

    let mut is_negative = false;
    if escaped_word.starts_with("meno") {
        is_negative = true;
        escaped_word = escaped_word.replace("meno", "");
    }

    if escaped_word == "zero" {
        return Ok(0);
    }

    if escaped_word.ends_with('a')
        || escaped_word.ends_with('o')
        || escaped_word.ends_with('e')
        || escaped_word.ends_with('i')
    {
        let index = ZERO_TEN_ORDINALS
            .iter()
            .position(|&ord| escaped_word.starts_with(&ord[..ord.len() - 1]));
        match index {
            Some(index) => {
                return Ok(index.try_into().unwrap());
            }
            None => {
                let ends = vec![
                    "esima", "esimo", "esime", "esimi", "decima", "decimo", "decime", "decimi",
                ];
                if ends.iter().any(|&end| escaped_word.ends_with(end)) {
                    escaped_word = get_number_from_ordinal(&escaped_word);
                }
            }
        }
    }

    match numbers_calculator(&escaped_word) {
        Ok(result) => {
            if is_negative {
                return Ok(-result);
            }

            Ok(result)
        }
        Err(_) => Err("invalid word number"),
    }
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::italian_converter;

    mod cardinal {
        use super::*;

        mod one_and_eight {
            use super::*;

            #[test]
            fn test_21() {
                assert_eq!(italian_converter("ventuno".to_string()), Ok(21))
            }

            #[test]
            fn test_28() {
                assert_eq!(italian_converter("ventotto".to_string()), Ok(28))
            }
        }

        mod exceptions {
            use super::*;

            #[test]
            fn test_blabla() {
                assert_eq!(
                    italian_converter("blabla".to_string()),
                    Err("invalid word number")
                )
            }

            #[test]
            fn test_ventottobla() {
                assert_eq!(
                    italian_converter("ventottobla".to_string()),
                    Err("invalid word number")
                )
            }

            #[test]
            fn test_centozero() {
                assert_eq!(
                    italian_converter("centozero".to_string()),
                    Err("invalid word number")
                )
            }
        }

        mod ones {
            use super::*;

            #[test]
            fn test_1() {
                assert_eq!(italian_converter("uno".to_string()), Ok(1));
            }

            #[test]
            fn test_100() {
                assert_eq!(italian_converter("cento".to_string()), Ok(100));
            }

            #[test]
            fn test_1_000() {
                assert_eq!(italian_converter("mille".to_string()), Ok(1_000));
            }

            #[test]
            fn test_1_000_000() {
                assert_eq!(italian_converter("un milione".to_string()), Ok(1_000_000));
            }

            #[test]
            fn test_1_000_000_000() {
                assert_eq!(
                    italian_converter("un miliardo".to_string()),
                    Ok(1_000_000_000)
                );
            }
        }

        mod threes {
            use super::*;

            #[test]
            fn test_3() {
                assert_eq!(italian_converter("tre".to_string()), Ok(3));
            }

            #[test]
            fn test_33() {
                assert_eq!(italian_converter("trentatré".to_string()), Ok(33));
            }

            #[test]
            fn test_333() {
                assert_eq!(italian_converter("trecentotrentatré".to_string()), Ok(333));
            }

            #[test]
            fn test_3_333() {
                assert_eq!(
                    italian_converter("tremilatrecentotrentatré".to_string()),
                    Ok(3_333)
                );
            }

            #[test]
            fn test_3_000_000() {
                assert_eq!(italian_converter("tre milioni".to_string()), Ok(3_000_000));
            }

            #[test]
            fn test_3_000_033() {
                assert_eq!(
                    italian_converter("tre milioni e trentatré".to_string()),
                    Ok(3_000_033)
                );
            }

            #[test]
            fn test_33_003_000() {
                assert_eq!(
                    italian_converter("trentatré milioni e tremila".to_string()),
                    Ok(33_003_000)
                );
            }

            #[test]
            fn test_3_033_000_000() {
                assert_eq!(
                    italian_converter("tre miliardi e trentatré milioni".to_string()),
                    Ok(3_033_000_000)
                );
            }

            #[test]
            fn test_3_003_000_000() {
                assert_eq!(
                    italian_converter("tre miliardi e tre milioni".to_string()),
                    Ok(3_003_000_000)
                );
            }

            #[test]
            fn test_3_000() {
                assert_eq!(italian_converter("tremila".to_string()), Ok(3_000));
            }

            #[test]
            fn test_23_000() {
                assert_eq!(italian_converter("ventitremila".to_string()), Ok(23_000));
            }

            #[test]
            fn test_103_103_103_103() {
                assert_eq!(
                    italian_converter(
                        "centotré miliardi e centotré milioni e centotremilacentotré".to_string()
                    ),
                    Ok(103_103_103_103)
                );
            }
        }

        mod min_max {
            use super::*;

            #[test]
            fn test_999_999_999_999() {
                assert_eq!(italian_converter("novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove".to_string()), Ok(999_999_999_999))
            }

            #[test]
            fn test_negative_999_999_999_999() {
                assert_eq!(italian_converter("meno novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove".to_string()), Ok(-999_999_999_999))
            }

            #[test]
            fn test_0() {
                assert_eq!(italian_converter("zero".to_string()), Ok(0))
            }
        }

        mod random {
            use super::*;

            #[test]
            fn test_27_347_687() {
                assert_eq!(
                    italian_converter(
                        "ventisette milioni e trecentoquarantasettemilaseicentottantasette"
                            .to_string()
                    ),
                    Ok(27_347_687)
                );
            }

            #[test]
            fn test_200_000_000() {
                assert_eq!(
                    italian_converter("duecento milioni".to_string()),
                    Ok(200_000_000)
                );
            }

            #[test]
            fn test_12_341() {
                assert_eq!(
                    italian_converter("dodicimilatrecentoquarantuno".to_string()),
                    Ok(12_341)
                );
            }

            #[test]
            fn test_negative_34_564() {
                assert_eq!(
                    italian_converter(
                        "meno trentaquattromilacinquecentosessantaquattro".to_string()
                    ),
                    Ok(-34_564)
                );
            }

            #[test]
            fn test_2_398_406() {
                assert_eq!(
                    italian_converter(
                        "due milioni e trecentonovantottomilaquattrocentosei".to_string()
                    ),
                    Ok(2_398_406)
                );
            }

            #[test]
            fn test_9_654_367() {
                assert_eq!(
                    italian_converter(
                        "nove milioni e seicentocinquantaquattromilatrecentosessantasette"
                            .to_string()
                    ),
                    Ok(9_654_367)
                );
            }

            #[test]
            fn test_100_100_100() {
                assert_eq!(
                    italian_converter("cento milioni e centomilacento".to_string()),
                    Ok(100_100_100)
                );
            }

            #[test]
            fn test_1013000() {
                assert_eq!(
                    italian_converter("un milione tredicimila".to_string()),
                    Ok(1013000)
                );
            }

            #[test]
            fn test_1010() {
                assert_eq!(italian_converter("milledieci".to_string()), Ok(1010));
            }
        }
    }

    mod ordinal {
        use super::*;

        mod first_thirty_numbers {
            use super::*;

            #[test]
            fn test_1() {
                assert_eq!(italian_converter("primo".to_string()), Ok(1))
            }

            #[test]
            fn test_2() {
                assert_eq!(italian_converter("secondo".to_string()), Ok(2))
            }

            #[test]
            fn test_3() {
                assert_eq!(italian_converter("terzo".to_string()), Ok(3))
            }

            #[test]
            fn test_4() {
                assert_eq!(italian_converter("quarto".to_string()), Ok(4))
            }

            #[test]
            fn test_5() {
                assert_eq!(italian_converter("quinto".to_string()), Ok(5))
            }

            #[test]
            fn test_6() {
                assert_eq!(italian_converter("sesto".to_string()), Ok(6))
            }

            #[test]
            fn test_7() {
                assert_eq!(italian_converter("settimo".to_string()), Ok(7))
            }

            #[test]
            fn test_8() {
                assert_eq!(italian_converter("ottavo".to_string()), Ok(8))
            }

            #[test]
            fn test_9() {
                assert_eq!(italian_converter("nono".to_string()), Ok(9))
            }

            #[test]
            fn test_10() {
                assert_eq!(italian_converter("decimo".to_string()), Ok(10))
            }

            #[test]
            fn test_11() {
                assert_eq!(italian_converter("undicesimo".to_string()), Ok(11))
            }

            #[test]
            fn test_12() {
                assert_eq!(italian_converter("dodicesimo".to_string()), Ok(12))
            }

            #[test]
            fn test_13() {
                assert_eq!(italian_converter("tredicesimo".to_string()), Ok(13))
            }

            #[test]
            fn test_14() {
                assert_eq!(italian_converter("quattordicesimo".to_string()), Ok(14))
            }

            #[test]
            fn test_15() {
                assert_eq!(italian_converter("quindicesimo".to_string()), Ok(15))
            }

            #[test]
            fn test_16() {
                assert_eq!(italian_converter("sedicesimo".to_string()), Ok(16))
            }

            #[test]
            fn test_17() {
                assert_eq!(italian_converter("diciassettesimo".to_string()), Ok(17))
            }

            #[test]
            fn test_18() {
                assert_eq!(italian_converter("diciottesimo".to_string()), Ok(18))
            }

            #[test]
            fn test_19() {
                assert_eq!(italian_converter("diciannovesimo".to_string()), Ok(19))
            }

            #[test]
            fn test_20() {
                assert_eq!(italian_converter("ventesimo".to_string()), Ok(20))
            }

            #[test]
            fn test_21() {
                assert_eq!(italian_converter("ventunesimo".to_string()), Ok(21))
            }

            #[test]
            fn test_22() {
                assert_eq!(italian_converter("ventiduesimo".to_string()), Ok(22))
            }

            #[test]
            fn test_23() {
                assert_eq!(italian_converter("ventitreesimo".to_string()), Ok(23))
            }

            #[test]
            fn test_24() {
                assert_eq!(italian_converter("ventiquattresimo".to_string()), Ok(24))
            }

            #[test]
            fn test_25() {
                assert_eq!(italian_converter("venticinquesimo".to_string()), Ok(25))
            }

            #[test]
            fn test_26() {
                assert_eq!(italian_converter("ventiseiesimo".to_string()), Ok(26))
            }

            #[test]
            fn test_27() {
                assert_eq!(italian_converter("ventisettesimo".to_string()), Ok(27))
            }

            #[test]
            fn test_28() {
                assert_eq!(italian_converter("ventottesimo".to_string()), Ok(28))
            }

            #[test]
            fn test_29() {
                assert_eq!(italian_converter("ventinovesimo".to_string()), Ok(29))
            }
        }

        mod first_thirty_female_numbers {
            use super::*;

            #[test]
            fn test_1() {
                assert_eq!(italian_converter("prima".to_string()), Ok(1))
            }

            #[test]
            fn test_2() {
                assert_eq!(italian_converter("seconda".to_string()), Ok(2))
            }

            #[test]
            fn test_3() {
                assert_eq!(italian_converter("terza".to_string()), Ok(3))
            }

            #[test]
            fn test_4() {
                assert_eq!(italian_converter("quarta".to_string()), Ok(4))
            }

            #[test]
            fn test_5() {
                assert_eq!(italian_converter("quinta".to_string()), Ok(5))
            }

            #[test]
            fn test_6() {
                assert_eq!(italian_converter("sesta".to_string()), Ok(6))
            }

            #[test]
            fn test_7() {
                assert_eq!(italian_converter("settima".to_string()), Ok(7))
            }

            #[test]
            fn test_8() {
                assert_eq!(italian_converter("ottava".to_string()), Ok(8))
            }

            #[test]
            fn test_9() {
                assert_eq!(italian_converter("nona".to_string()), Ok(9))
            }

            #[test]
            fn test_10() {
                assert_eq!(italian_converter("decima".to_string()), Ok(10))
            }

            #[test]
            fn test_11() {
                assert_eq!(italian_converter("undicesima".to_string()), Ok(11))
            }

            #[test]
            fn test_12() {
                assert_eq!(italian_converter("dodicesima".to_string()), Ok(12))
            }

            #[test]
            fn test_13() {
                assert_eq!(italian_converter("tredicesima".to_string()), Ok(13))
            }

            #[test]
            fn test_14() {
                assert_eq!(italian_converter("quattordicesima".to_string()), Ok(14))
            }

            #[test]
            fn test_15() {
                assert_eq!(italian_converter("quindicesima".to_string()), Ok(15))
            }

            #[test]
            fn test_16() {
                assert_eq!(italian_converter("sedicesima".to_string()), Ok(16))
            }

            #[test]
            fn test_17() {
                assert_eq!(italian_converter("diciassettesima".to_string()), Ok(17))
            }

            #[test]
            fn test_18() {
                assert_eq!(italian_converter("diciottesima".to_string()), Ok(18))
            }

            #[test]
            fn test_19() {
                assert_eq!(italian_converter("diciannovesima".to_string()), Ok(19))
            }

            #[test]
            fn test_20() {
                assert_eq!(italian_converter("ventesima".to_string()), Ok(20))
            }

            #[test]
            fn test_21() {
                assert_eq!(italian_converter("ventunesima".to_string()), Ok(21))
            }

            #[test]
            fn test_22() {
                assert_eq!(italian_converter("ventiduesima".to_string()), Ok(22))
            }

            #[test]
            fn test_23() {
                assert_eq!(italian_converter("ventitreesima".to_string()), Ok(23))
            }

            #[test]
            fn test_24() {
                assert_eq!(italian_converter("ventiquattresima".to_string()), Ok(24))
            }

            #[test]
            fn test_25() {
                assert_eq!(italian_converter("venticinquesima".to_string()), Ok(25))
            }

            #[test]
            fn test_26() {
                assert_eq!(italian_converter("ventiseiesima".to_string()), Ok(26))
            }

            #[test]
            fn test_27() {
                assert_eq!(italian_converter("ventisettesima".to_string()), Ok(27))
            }

            #[test]
            fn test_28() {
                assert_eq!(italian_converter("ventottesima".to_string()), Ok(28))
            }

            #[test]
            fn test_29() {
                assert_eq!(italian_converter("ventinovesima".to_string()), Ok(29))
            }
        }

        mod first_thirty_plural_numbers {
            use super::*;

            #[test]
            fn test_1() {
                assert_eq!(italian_converter("primi".to_string()), Ok(1))
            }

            #[test]
            fn test_2() {
                assert_eq!(italian_converter("secondi".to_string()), Ok(2))
            }

            #[test]
            fn test_3() {
                assert_eq!(italian_converter("terzi".to_string()), Ok(3))
            }

            #[test]
            fn test_4() {
                assert_eq!(italian_converter("quarti".to_string()), Ok(4))
            }

            #[test]
            fn test_5() {
                assert_eq!(italian_converter("quinti".to_string()), Ok(5))
            }

            #[test]
            fn test_6() {
                assert_eq!(italian_converter("sesti".to_string()), Ok(6))
            }

            #[test]
            fn test_7() {
                assert_eq!(italian_converter("settimi".to_string()), Ok(7))
            }

            #[test]
            fn test_8() {
                assert_eq!(italian_converter("ottavi".to_string()), Ok(8))
            }

            #[test]
            fn test_9() {
                assert_eq!(italian_converter("noni".to_string()), Ok(9))
            }

            #[test]
            fn test_10() {
                assert_eq!(italian_converter("decimi".to_string()), Ok(10))
            }

            #[test]
            fn test_11() {
                assert_eq!(italian_converter("undicesimi".to_string()), Ok(11))
            }

            #[test]
            fn test_12() {
                assert_eq!(italian_converter("dodicesimi".to_string()), Ok(12))
            }

            #[test]
            fn test_13() {
                assert_eq!(italian_converter("tredicesimi".to_string()), Ok(13))
            }

            #[test]
            fn test_14() {
                assert_eq!(italian_converter("quattordicesimi".to_string()), Ok(14))
            }

            #[test]
            fn test_15() {
                assert_eq!(italian_converter("quindicesimi".to_string()), Ok(15))
            }

            #[test]
            fn test_16() {
                assert_eq!(italian_converter("sedicesimi".to_string()), Ok(16))
            }

            #[test]
            fn test_17() {
                assert_eq!(italian_converter("diciassettesimi".to_string()), Ok(17))
            }

            #[test]
            fn test_18() {
                assert_eq!(italian_converter("diciottesimi".to_string()), Ok(18))
            }

            #[test]
            fn test_19() {
                assert_eq!(italian_converter("diciannovesimi".to_string()), Ok(19))
            }

            #[test]
            fn test_20() {
                assert_eq!(italian_converter("ventesimi".to_string()), Ok(20))
            }

            #[test]
            fn test_21() {
                assert_eq!(italian_converter("ventunesimi".to_string()), Ok(21))
            }

            #[test]
            fn test_22() {
                assert_eq!(italian_converter("ventiduesimi".to_string()), Ok(22))
            }

            #[test]
            fn test_23() {
                assert_eq!(italian_converter("ventitreesimi".to_string()), Ok(23))
            }

            #[test]
            fn test_24() {
                assert_eq!(italian_converter("ventiquattresimi".to_string()), Ok(24))
            }

            #[test]
            fn test_25() {
                assert_eq!(italian_converter("venticinquesimi".to_string()), Ok(25))
            }

            #[test]
            fn test_26() {
                assert_eq!(italian_converter("ventiseiesimi".to_string()), Ok(26))
            }

            #[test]
            fn test_27() {
                assert_eq!(italian_converter("ventisettesimi".to_string()), Ok(27))
            }

            #[test]
            fn test_28() {
                assert_eq!(italian_converter("ventottesimi".to_string()), Ok(28))
            }

            #[test]
            fn test_29() {
                assert_eq!(italian_converter("ventinovesimi".to_string()), Ok(29))
            }
        }

        mod first_thirty_plural_female_numbers {
            use super::*;

            #[test]
            fn test_1() {
                assert_eq!(italian_converter("prime".to_string()), Ok(1))
            }

            #[test]
            fn test_2() {
                assert_eq!(italian_converter("seconde".to_string()), Ok(2))
            }

            #[test]
            fn test_3() {
                assert_eq!(italian_converter("terze".to_string()), Ok(3))
            }

            #[test]
            fn test_4() {
                assert_eq!(italian_converter("quarte".to_string()), Ok(4))
            }

            #[test]
            fn test_5() {
                assert_eq!(italian_converter("quinte".to_string()), Ok(5))
            }

            #[test]
            fn test_6() {
                assert_eq!(italian_converter("seste".to_string()), Ok(6))
            }

            #[test]
            fn test_7() {
                assert_eq!(italian_converter("settime".to_string()), Ok(7))
            }

            #[test]
            fn test_8() {
                assert_eq!(italian_converter("ottave".to_string()), Ok(8))
            }

            #[test]
            fn test_9() {
                assert_eq!(italian_converter("none".to_string()), Ok(9))
            }

            #[test]
            fn test_10() {
                assert_eq!(italian_converter("decime".to_string()), Ok(10))
            }

            #[test]
            fn test_11() {
                assert_eq!(italian_converter("undicesime".to_string()), Ok(11))
            }

            #[test]
            fn test_12() {
                assert_eq!(italian_converter("dodicesime".to_string()), Ok(12))
            }

            #[test]
            fn test_13() {
                assert_eq!(italian_converter("tredicesime".to_string()), Ok(13))
            }

            #[test]
            fn test_14() {
                assert_eq!(italian_converter("quattordicesime".to_string()), Ok(14))
            }

            #[test]
            fn test_15() {
                assert_eq!(italian_converter("quindicesime".to_string()), Ok(15))
            }

            #[test]
            fn test_16() {
                assert_eq!(italian_converter("sedicesime".to_string()), Ok(16))
            }

            #[test]
            fn test_17() {
                assert_eq!(italian_converter("diciassettesime".to_string()), Ok(17))
            }

            #[test]
            fn test_18() {
                assert_eq!(italian_converter("diciottesime".to_string()), Ok(18))
            }

            #[test]
            fn test_19() {
                assert_eq!(italian_converter("diciannovesime".to_string()), Ok(19))
            }

            #[test]
            fn test_20() {
                assert_eq!(italian_converter("ventesime".to_string()), Ok(20))
            }

            #[test]
            fn test_21() {
                assert_eq!(italian_converter("ventunesime".to_string()), Ok(21))
            }

            #[test]
            fn test_22() {
                assert_eq!(italian_converter("ventiduesime".to_string()), Ok(22))
            }

            #[test]
            fn test_23() {
                assert_eq!(italian_converter("ventitreesime".to_string()), Ok(23))
            }

            #[test]
            fn test_24() {
                assert_eq!(italian_converter("ventiquattresime".to_string()), Ok(24))
            }

            #[test]
            fn test_25() {
                assert_eq!(italian_converter("venticinquesime".to_string()), Ok(25))
            }

            #[test]
            fn test_26() {
                assert_eq!(italian_converter("ventiseiesime".to_string()), Ok(26))
            }

            #[test]
            fn test_27() {
                assert_eq!(italian_converter("ventisettesime".to_string()), Ok(27))
            }

            #[test]
            fn test_28() {
                assert_eq!(italian_converter("ventottesime".to_string()), Ok(28))
            }

            #[test]
            fn test_29() {
                assert_eq!(italian_converter("ventinovesime".to_string()), Ok(29))
            }
        }

        mod min_max {
            use super::*;

            #[test]
            fn test_999_999_999_999() {
                assert_eq!(italian_converter("novecentonovantanovemiliardinovecentonovantanovemilioninovecentonovantanovemilanovecentonovantanovesimo".to_string()), Ok(999_999_999_999))
            }

            #[test]
            fn test_0() {
                assert_eq!(italian_converter("zeresimo".to_string()), Ok(0))
            }
        }

        mod hundreds {
            use super::*;

            #[test]
            fn test_101() {
                assert_eq!(italian_converter("centunesimo".to_string()), Ok(101))
            }

            #[test]
            fn test_108() {
                assert_eq!(italian_converter("centottesimo".to_string()), Ok(108))
            }

            #[test]
            fn test_110() {
                assert_eq!(italian_converter("centodecimo".to_string()), Ok(110))
            }

            #[test]
            fn test_116() {
                assert_eq!(italian_converter("centosedicesimo".to_string()), Ok(116))
            }
        }

        mod tens {
            use super::*;

            #[test]
            fn test_910() {
                assert_eq!(italian_converter("novecentodecimo".to_string()), Ok(910));
            }

            #[test]
            fn test_920() {
                assert_eq!(italian_converter("novecentoventesimo".to_string()), Ok(920));
            }

            #[test]
            fn test_930() {
                assert_eq!(
                    italian_converter("novecentotrentesimo".to_string()),
                    Ok(930)
                );
            }

            #[test]
            fn test_940() {
                assert_eq!(
                    italian_converter("novecentoquarantesimo".to_string()),
                    Ok(940)
                );
            }

            #[test]
            fn test_950() {
                assert_eq!(
                    italian_converter("novecentocinquantesimo".to_string()),
                    Ok(950)
                );
            }

            #[test]
            fn test_960() {
                assert_eq!(
                    italian_converter("novecentosessantesimo".to_string()),
                    Ok(960)
                );
            }

            #[test]
            fn test_970() {
                assert_eq!(
                    italian_converter("novecentosettantesimo".to_string()),
                    Ok(970)
                );
            }

            #[test]
            fn test_980() {
                assert_eq!(
                    italian_converter("novecentoottantesimo".to_string()),
                    Ok(980)
                );
            }

            #[test]
            fn test_990() {
                assert_eq!(
                    italian_converter("novecentonovantesimo".to_string()),
                    Ok(990)
                );
            }
        }

        mod random {
            use super::*;

            #[test]
            fn test_801_100() {
                assert_eq!(
                    italian_converter("ottocentounomilacentesimo".to_string()),
                    Ok(801_100)
                );
            }

            #[test]
            fn test_108_416() {
                assert_eq!(
                    italian_converter("centoottomilaquattrocentosedicesimo".to_string()),
                    Ok(108_416)
                );
            }

            #[test]
            fn test_1_000() {
                assert_eq!(italian_converter("millesimo".to_string()), Ok(1_000));
            }

            #[test]
            fn test_1_110() {
                assert_eq!(italian_converter("millecentodecimo".to_string()), Ok(1_110));
            }

            #[test]
            fn test_1_000_000() {
                assert_eq!(italian_converter("milionesimo".to_string()), Ok(1_000_000));
            }

            #[test]
            fn test_1_000_000_000() {
                assert_eq!(
                    italian_converter("miliardesimo".to_string()),
                    Ok(1_000_000_000)
                );
            }

            #[test]
            fn test_1_000_001() {
                assert_eq!(
                    italian_converter("milioneunesimo".to_string()),
                    Ok(1_000_001)
                );
            }

            #[test]
            fn test_1_000_000_001() {
                assert_eq!(
                    italian_converter("miliardunesimo".to_string()),
                    Ok(1_000_000_001)
                );
            }

            #[test]
            fn test_1_000_000_008() {
                assert_eq!(
                    italian_converter("miliardottesimo".to_string()),
                    Ok(1_000_000_008)
                );
            }

            #[test]
            fn test_2_000() {
                assert_eq!(italian_converter("duemillesimo".to_string()), Ok(2_000));
            }

            #[test]
            fn test_2_000_000_001() {
                assert_eq!(
                    italian_converter("duemiliardunesimo".to_string()),
                    Ok(2_000_000_001)
                );
            }

            #[test]
            fn test_456_799_123() {
                assert_eq!(italian_converter("quattrocentocinquantaseimilionisettecentonovantanovemilacentoventitreesimo".to_string()), Ok(456_799_123));
            }

            #[test]
            fn test_103() {
                assert_eq!(italian_converter("centotreesimo".to_string()), Ok(103));
            }
        }
    }

    // mod mixed {
    //     use crate::ordinal_converter;

    //     use super::*;

    //     #[test]
    //     fn test_all() {
    //         for i in 0..=1000 {
    //             let ordinal_number = ordinal_converter(i, None).unwrap();
    //             assert_eq!(italian_converter(ordinal_number).unwrap(), i.try_into().unwrap())
    //         }
    //     }
    // }
}
