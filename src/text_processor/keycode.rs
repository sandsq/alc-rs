#[derive(Debug, PartialEq)]
pub enum Keycode {
	_A,
	_B,
	_C,
	_D,
	_E,
	_SFT,
	_PLACEHOLDER,
}
use Keycode::*;

pub fn char_to_keycode(c: char) -> Vec<Keycode> {
	let mut keycodes: Vec<Keycode> = vec![];
	if c.is_uppercase() {
		keycodes.push(_SFT);
	}
	match c.to_lowercase().next().unwrap() {
		'a' => keycodes.push(_A),
		'b' => keycodes.push(_B),
		'c' => keycodes.push(_C),
		'd' => keycodes.push(_D),
		'e' => keycodes.push(_E),
		_ => keycodes.push(_PLACEHOLDER),
	};
	keycodes
}

pub fn string_to_keycode(s: &str) -> Vec<Keycode> {
	let mut keycodes: Vec<Keycode> = vec![];
	for c in s.chars() {
		keycodes.append(&mut char_to_keycode(c));
	}
	keycodes
}



#[cfg(test)]
mod tests {
	use super::*;

	const DUMMY_STR: &str = "Aaaaabbbb ccc
							 dd e";

	#[test]
	fn a_to_keycode() {
		let res: Vec<Keycode> = vec![_A];
		assert_eq!(char_to_keycode('a'), res);
	}

	#[test]
	fn cap_e_to_keycode() {
		let res: Vec<Keycode> = vec![_SFT, _E];
		assert_eq!(char_to_keycode('E'), res);
	}

	#[test]
	fn acb_to_keycodes() {
		let res: Vec<Keycode> = vec![_A, _SFT, _C, _B];
		assert_eq!(string_to_keycode("aCb"), res);
	}
}
