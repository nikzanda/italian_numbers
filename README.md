# italian_numbers

## Purpose

### Converts a number to an italian word representation and vice versa

Works with positive and negative integers from **-999999999999** (meno novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove) to **999999999999** (novecentonovantanove miliardi e novecentonovantanove milioni e novecentonovantanovemilanovecentonovantanove).

The representation can be a cardinal number (uno, due, tre...) or an ordinal number (primo, secondo, terzo). Ordinal numbers can also be translated in feminine form (prima, seconda, terza...), in plural form (primi, secondi, terzi...) or plural feminine (prime, seconde, terze...).

<!-- Also works with `Infinity` (infinito). -->

### Converts an Arabic number to a Roman number and vice versa

Works with positive integers from **1** (I) to **3999** (MMMCMXCIX).

<!-- Also works with `Infinity` (infinitum). -->

## Installation

Install using

```sh
cargo add italian_numbers
```

## Usage

```rust
use italian_numbers::roman_converter{cardinal_converter, ordinal_converter, roman_converter, arabic_converter, Options};

// Cardinal numbers representation
cardinal_converter(1); // "uno"
cardinal_converter(90); // "novanta"
cardinal_converter(709); // "settecentonove"

// Ordinal numbers representation
ordinal_converter(1); // "primo"
ordinal_converter(10); // "decimo"
ordinal_converter(63); // "sessantatreesimo"

ordinal_converter(1, Options::new(true, false)); // "prima"
ordinal_converter(15, Options::new(true, false)); // "quindicesima"
ordinal_converter(109, Options::new(true, false)); // "centonovesima"

ordinal_converter(1, Options::new(false, true)); // "primi"
ordinal_converter(18, Options::new(false, true)); // "diciottesimi"
ordinal_converter(89, Options::new(false, true)); // "ottantanovesimi"

ordinal_converter(1, Options::new(true, true)); // "prime"
ordinal_converter(70, Options::new(true, true)); // "settantesime"
ordinal_converter(110, Options::new(true, true)); // "centodecime"

// Arabic to Roman
roman_converter(1); // "I"
roman_converter(79); // "LXXIX"
roman_converter(2_317); // "MMCCCXVII"

// Roman to Arabic
arabic_converter(String::from("MD")); // 1_500
arabic_converter(String::from("CDXC")); // 490
arabic_converter(String::from("MCMXC")); // 1_990

// Italian word representation to number
italian_converter(String::from("uno")); // 1
italian_converter(String::from("novantasette")); // 97
italian_converter(String::from("un milione tredicimila")); // 1_013_000
italian_converter(String::from("zeresimo")); // 0
italian_converter(String::from("prima")); // 1
italian_converter(String::from("quattrocentotredicesime")); // 413
```