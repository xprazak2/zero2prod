use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::SubscriberName;
  use claim::{assert_err, assert_ok};

  #[test]
  fn a_256_grapheme_long_name_is_valid() {
    let name = "a".repeat(256);
    assert_ok!(SubscriberName::parse(name));
  }

  #[test]
  fn a_name_longer_than_256_graphemes_is_rejected() {
    let name = "a".repeat(257);
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn whitespace_only_names_are_rejected() {
    let name = " ".to_string();
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn empty_string_is_rejected() {
    let name = "".to_string();
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn names_containing_an_invalid_character_are_rejected() {
    for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
      let name = name.to_string();
      assert_err!(SubscriberName::parse(name));
    }
  }

  #[test]
  fn a_valid_name_is_parsed_successfully() {
    let name = "Ursula Le Guin".to_string();
    assert_ok!(SubscriberName::parse(name));
  }
}
