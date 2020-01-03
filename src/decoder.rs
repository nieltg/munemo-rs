use crate::constants::*;

pub fn decode(mut code: &str) -> Option<i64> {
  let mut value: i64 = 0;
  let negative_factor = if code.starts_with(NEGATIVE_SYLLABLE) {
    code = &code[NEGATIVE_SYLLABLE.len()..];
    -1
  } else {
    1
  };

  while code.len() > 0 {
    match SYLLABLES
      .iter()
      .position(|&syllable| code.starts_with(syllable))
    {
      Some(index) => {
        value *= SYLLABLES.len() as i64;
        value += index as i64;
        code = &code[SYLLABLES[index].len()..];
      }
      None => return None,
    };
  }
  Some(value * negative_factor)
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  #[test]
  fn single_syllable() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = units as i64;

    assert_eq!(decode(SYLLABLES[units]).unwrap(), value);
  }

  #[test]
  fn unknown_syllable() {
    assert_eq!(decode("aa"), None);
  }

  #[test]
  fn many_tens() {
    let tens = rand::thread_rng().gen_range(1, SYLLABLES.len());
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (tens * SYLLABLES.len() + units) as i64;

    assert_eq!(
      decode(&[SYLLABLES[tens], SYLLABLES[units]].concat()).unwrap(),
      value
    );
  }

  #[test]
  fn many_hundreds() {
    let hundreds = rand::thread_rng().gen_range(1, SYLLABLES.len());
    let tens = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value =
      (hundreds * SYLLABLES.len() * SYLLABLES.len() + tens * SYLLABLES.len() + units) as i64;

    assert_eq!(
      decode(&[SYLLABLES[hundreds], SYLLABLES[tens], SYLLABLES[units]].concat()).unwrap(),
      value
    );
  }

  #[test]
  fn negative() {
    let units = rand::thread_rng().gen_range(0, SYLLABLES.len());
    let value = (units as i64) * -1;

    assert_eq!(
      decode(&[NEGATIVE_SYLLABLE, SYLLABLES[units]].concat()).unwrap(),
      value
    );
  }
}
