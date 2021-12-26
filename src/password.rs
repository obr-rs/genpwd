use crate::config::Minimum;
use rand::prelude::*;

/// Generates the password
pub fn generate(minimum: &Minimum) -> String {
	let mut random = thread_rng();
	let mut occurrences: [u16; 4] = [0, 0, 0, 0];
	let mut fulfilled: u8 = 0;
	let mut password = String::from("");
	let char_classes = new_char_classes();
	if minimum.occurrence >= 1 {
		while fulfilled < 4 {
			let index_char_classes = random.gen_range(0..char_classes.len());
			if occurrences[index_char_classes] < minimum.occurrence {
				let index_char = random.gen_range(0..char_classes[index_char_classes].len());
				password.push_str(&char_classes[index_char_classes][index_char..index_char + 1]);
				occurrences[index_char_classes] += 1;
				if occurrences[index_char_classes] == minimum.occurrence {
					fulfilled += 1;
				}
			}
		}
	}
	while password.len() < minimum.length {
		let index_char_classes = random.gen_range(0..char_classes.len());
		let index_char = random.gen_range(0..char_classes[index_char_classes].len());
		password.push_str(&char_classes[index_char_classes][index_char..index_char + 1]);
		occurrences[index_char_classes] += 1;
	}
	password
}

/// Creates an array with character classes
fn new_char_classes() -> [&'static str; 4] {
	[
		"abcdefghijklmnopqrstuvwxyz",
		"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		"0123456789",
		"!\"$%&/?\\(){}[]+*~#'<>|,;.:-_",
	]
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_char_classes() {
		let char_classes = new_char_classes();
		for char_class in char_classes.iter() {
			assert_eq!(char_class.len(), char_class.chars().count());
		}
	}
	#[test]
	fn test_length() {
		let minimum = Minimum {
			occurrence: 0,
			length: 10,
		};
		let result = generate(&minimum);
		assert_eq!(result.len(), 10);
	}
	#[test]
	fn test_occurences() {
		let minimum = Minimum {
			occurrence: 4,
			length: 10,
		};
		let result = generate(&minimum);
		assert_eq!(result.len(), 16);
		let mut occurrences: [u16; 4] = [0, 0, 0, 0];
		let char_classes = new_char_classes();
		for index in 0..result.len() {
			let character = &result[index..index + 1];
			if char_classes[0].contains(character) {
				occurrences[0] += 1;
			} else if char_classes[1].contains(character) {
				occurrences[1] += 1;
			} else if char_classes[2].contains(character) {
				occurrences[2] += 1;
			} else if char_classes[3].contains(character) {
				occurrences[3] += 1;
			}
		}
		assert_eq!(occurrences, [4, 4, 4, 4]);
	}
}
