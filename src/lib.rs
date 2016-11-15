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

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;

#[cfg(test)]
mod tests {
	use super::send_str;

	#[test]
	fn test_lowercase_str() {
		send_str("echo");
	}
}
