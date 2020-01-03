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
const NEGATIVE_SYLLABLE: &str = "xa";

fn encode_unit(index: usize, buffer: &mut String) {
  let tens = index / SYLLABLES.len();
  if tens > 0 {
    encode_unit(tens, buffer);
  }

  let units = index % SYLLABLES.len();
  buffer.push_str(SYLLABLES[units]);
}

pub fn encode(value: i64) -> String {
  let mut buffer = String::new();
  let index: usize = if value < 0 {
    buffer.push_str(NEGATIVE_SYLLABLE);
    -1 * value
  } else {
    value
  }
  .try_into()
  .unwrap();

  encode_unit(index, &mut buffer);
  buffer
}

pub fn decode(code: &str) -> Option<i64> {
  Some(SYLLABLES.iter().position(|&syllable| syllable == code).unwrap() as i64)
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

  #[test]
  fn encode_many_hundreds() {
    let hundreds = rand::thread_rng().gen_range(1, SYLLABLES.len());
    let tens = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value =
      (hundreds * SYLLABLES.len() * SYLLABLES.len() + tens * SYLLABLES.len() + units) as i64;

    assert_eq!(
      encode(value),
      [SYLLABLES[hundreds], SYLLABLES[tens], SYLLABLES[units]].concat()
    );
  }

  #[test]
  fn encode_negative() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (units as i64) * -1;

    assert_eq!(
      encode(value),
      [NEGATIVE_SYLLABLE, SYLLABLES[units]].concat()
    );
  }

  #[test]
  fn decode_single_syllable() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = units as i64;

    assert_eq!(decode(SYLLABLES[units]).unwrap(), value);
  }
}
