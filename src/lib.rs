use std::convert::TryInto;

const SYLLABLES: [&str; 100] = [
  "ba", "bi", "bu", "be", "bo", "cha", "chi", "chu", "che", "cho", "da", "di", "du", "de", "do",
  "fa", "fi", "fu", "fe", "fo", "ga", "gi", "gu", "ge", "go", "ha", "hi", "hu", "he", "ho", "ja",
  "ji", "ju", "je", "jo", "ka", "ki", "ku", "ke", "ko", "la", "li", "lu", "le", "lo", "ma", "mi",
  "mu", "me", "mo", "na", "ni", "nu", "ne", "no", "pa", "pi", "pu", "pe", "po", "ra", "ri", "ru",
  "re", "ro", "sa", "si", "su", "se", "so", "sha", "shi", "shu", "she", "sho", "ta", "ti", "tu",
  "te", "to", "tsa", "tsi", "tsu", "tse", "tso", "wa", "wi", "wu", "we", "wo", "ya", "yi", "yu",
  "ye", "yo", "za", "zi", "zu", "ze", "zo",
];

pub fn encode(value: i64) -> String {
  let mut index: usize = value.try_into().unwrap();
  let mut buffer = String::new();

  if index >= SYLLABLES.len() {
    buffer.push_str(SYLLABLES[index / SYLLABLES.len()]);
    index %= SYLLABLES.len();
  }
  buffer.push_str(SYLLABLES[index]);
  buffer
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  #[test]
  fn encode_single_syllable() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = units as i64;

    assert_eq!(encode(value), SYLLABLES[units]);
  }

  #[test]
  fn encode_one_tens() {
    let tens = 1;
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (tens * SYLLABLES.len() + units) as i64;

    assert_eq!(encode(value), [SYLLABLES[tens], SYLLABLES[units]].concat());
  }

  #[test]
  fn encode_many_tens() {
    let tens = rand::thread_rng().gen_range(2, SYLLABLES.len());
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (tens * SYLLABLES.len() + units) as i64;

    assert_eq!(encode(value), [SYLLABLES[tens], SYLLABLES[units]].concat());
  }
}
