mod cardinal_converter;
mod ordinal_converter;
mod roman_converter;
mod arabic_converter;
mod italian_converter;

pub(crate) const ZERO_NINETEEN: [&str; 20] = [
    "zero",
    "uno",
    "due",
    "tre",
    "quattro",
    "cinque",
    "sei",
    "sette",
    "otto",
    "nove",
    "dieci",
    "undici",
    "dodici",
    "tredici",
    "quattordici",
    "quindici",
    "sedici",
    "diciassette",
    "diciotto",
    "diciannove",
];

pub(crate) const ZERO_TEN_ORDINALS: [&str; 11] = [
    "zeresimo",
    "primo",
    "secondo",
    "terzo",
    "quarto",
    "quinto",
    "sesto",
    "settimo",
    "ottavo",
    "nono",
    "decimo",
];

pub(crate) const TENS: [&str; 8] = [
    "venti",
    "trenta",
    "quaranta",
    "cinquanta",
    "sessanta",
    "settanta",
    "ottanta",
    "novanta",
];

pub(crate) const HUNDRED: &str = "cento";
pub(crate) const THOUSANDS: [&str; 2] = ["mille", "mila"];
pub(crate) const MILLIONS: [&str; 2] = ["un milione", " milioni"];
pub(crate) const BILLIONS: [&str; 2] = ["un miliardo", " miliardi"];

pub(crate) const AND: &str = " e ";

pub(crate) const ROMAN_UNITS: [&str; 10] = [
    "",
    "I",
    "II",
    "III",
    "IV",
    "V",
    "VI",
    "VII",
    "VIII",
    "IX",
];

pub(crate) const ROMAN_TENS: [&str; 10] = [
    "",
    "X",
    "XX",
    "XXX",
    "XL",
    "L",
    "LX",
    "LXX",
    "LXXX",
    "XC",
];

pub(crate) const ROMAN_HUNDREDS: [&str; 10] = [
    "",
    "C",
    "CC",
    "CCC",
    "CD",
    "D",
    "DC",
    "DCC",
    "DCCC",
    "CM",
];

pub(crate) const ROMAN_THOUSANDS: [&str; 4] = [
    "",
    "M",
    "MM",
    "MMM",
];

pub(crate) const ROMAN_LETTERS: [&str; 7] = [
    "I",
    "V",
    "X",
    "L",
    "C",
    "D",
    "M"
];

pub use cardinal_converter::cardinal_converter;
pub use ordinal_converter::{ordinal_converter, Options};
pub use roman_converter::roman_converter;
pub use arabic_converter::arabic_converter;
pub use italian_converter::italian_converter;