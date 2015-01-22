//! Send a string, character, or keystroke event to the system.

pub use platform::{press_key, release_key};
pub use platform::{send_key, send_combo};
pub use platform::{send_char, send_str};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Physical {
	Return,
	Control,
	Alt,
	Shift,
	A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Key {
	Physical(Physical),
	Unicode(char),
}

#[cfg(target_os = "macos")]
mod platform {

}

#[cfg(target_os = "windows")]
mod platform {
	extern crate winapi;
	extern crate "user32-sys" as user32_sys;

	use std::mem::{size_of, transmute_copy};
	use self::winapi::{c_int, WORD};
	use self::winapi::{INPUT_KEYBOARD, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE};
	use self::winapi::{INPUT, LPINPUT, KEYBDINPUT, MOUSEINPUT};
	use self::winapi::{VK_RETURN, VK_SHIFT, VK_CONTROL, VK_MENU};
	use self::user32_sys::SendInput;

	use super::{Physical, Key};

	fn get_keycode(p: Physical) -> WORD {
		use Physical::*;
		match p {
			Return => VK_RETURN as WORD,
			Shift => VK_SHIFT as WORD,
			Control => VK_CONTROL as WORD,
			Alt => VK_MENU as WORD,
			A => 'A' as WORD,
			B => 'B' as WORD,
			C => 'C' as WORD,
			D => 'D' as WORD,
			E => 'E' as WORD,
			F => 'F' as WORD,
			G => 'G' as WORD,
			H => 'H' as WORD,
			I => 'I' as WORD,
			J => 'J' as WORD,
			K => 'K' as WORD,
			L => 'L' as WORD,
			M => 'M' as WORD,
			N => 'N' as WORD,
			O => 'O' as WORD,
			P => 'P' as WORD,
			Q => 'Q' as WORD,
			R => 'R' as WORD,
			S => 'S' as WORD,
			T => 'T' as WORD,
			U => 'U' as WORD,
			V => 'V' as WORD,
			W => 'W' as WORD,
			X => 'X' as WORD,
			Y => 'Y' as WORD,
			Z => 'Z' as WORD,
		}
	}

	pub fn press_key(k: Key) {
		unsafe { match k {
			Key::Physical(p) => {
				let mut x = INPUT {
					type_: INPUT_KEYBOARD,
					union_: transmute_copy::<KEYBDINPUT, MOUSEINPUT>(&KEYBDINPUT {
						wVk: get_keycode(p), // 'a' key
						wScan: 0, // 0 := hardware scan code for a key
						dwFlags: 0, // 0 := a key press
						time: 0,
						dwExtraInfo: 0,
					}),
				};
				SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
			},
			Key::Unicode(c) => {
				let mut x = INPUT {
					type_: INPUT_KEYBOARD,
					union_: transmute_copy::<KEYBDINPUT, MOUSEINPUT>(&KEYBDINPUT {
						wVk: 0,
						wScan: c as WORD, // 0 := hardware scan code for a key
						dwFlags: KEYEVENTF_UNICODE, // 0 := a key press
						time: 0,
						dwExtraInfo: 0,
					}),
				};
				SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
			}
		}}
	}

	pub fn release_key(k: Key) {
		unsafe { match k {
			Key::Physical(p) => {
				let mut x = INPUT {
					type_: INPUT_KEYBOARD,
					union_: transmute_copy::<KEYBDINPUT, MOUSEINPUT>(&KEYBDINPUT {
						wVk: get_keycode(p), // 'a' key
						wScan: 0, // 0 := hardware scan code for a key
						dwFlags: KEYEVENTF_KEYUP,
						time: 0,
						dwExtraInfo: 0,
					}),
				};
				SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
			},
			Key::Unicode(c) => {
				let mut x = INPUT {
					type_: INPUT_KEYBOARD,
					union_: transmute_copy::<KEYBDINPUT, MOUSEINPUT>(&KEYBDINPUT {
						wVk: 0, // 'a' key
						wScan: c as WORD, // 0 := hardware scan code for a key
						dwFlags: KEYEVENTF_UNICODE|KEYEVENTF_KEYUP,
						time: 0,
						dwExtraInfo: 0,
					}),
				};
				SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as c_int);
			}
		}}
	}

	pub fn send_combo(keys: &[Key]) {
		for &k in keys.iter() {
			press_key(k);
		}
		for &k in keys.iter().rev() {
			release_key(k);
		}
	}

	pub fn send_key(k: Key) {
		press_key(k);
		release_key(k);
	}

	/// Send all unicode characters below 0x10000, silently skipping others.
	pub fn send_char(c: char) {
		if (c as u64) < 0x10000 {
			send_key(Key::Unicode(c));
		}
	}

	/// Send a string as keyboard events
	pub fn send_str(msg: &str) {
		for c in msg.chars() {
			send_char(c);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::send_str;

	#[test]
	fn test_lowercase_str() {
		send_str("echo");
	}
}
