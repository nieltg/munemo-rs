use crate::constants::*;
use std::convert::TryInto;

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

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  #[test]
  fn single_syllable() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = units as i64;

    assert_eq!(encode(value), SYLLABLES[units]);
  }

  #[test]
  fn one_tens() {
    let tens = 1;
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (tens * SYLLABLES.len() + units) as i64;

    assert_eq!(encode(value), [SYLLABLES[tens], SYLLABLES[units]].concat());
  }

  #[test]
  fn many_tens() {
    let tens = rand::thread_rng().gen_range(2, SYLLABLES.len());
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (tens * SYLLABLES.len() + units) as i64;

    assert_eq!(encode(value), [SYLLABLES[tens], SYLLABLES[units]].concat());
  }

  #[test]
  fn many_hundreds() {
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
  fn negative() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (units as i64) * -1;

    assert_eq!(
      encode(value),
      [NEGATIVE_SYLLABLE, SYLLABLES[units]].concat()
    );
  }
}
