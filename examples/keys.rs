extern crate keystroke;

// simple
use keystroke::{send_char, send_str};
// medium
use keystroke::{send_key, send_combo, Key, Physical};
// complicated
use keystroke::{press_key, release_key};

fn main() {
	// simple
	send_str("echo FOO bar\n");
	send_char('\n');

	// medium
	send_combo(&[
		Key::Physical(Physical::E), Key::Unicode('c'), Key::Unicode('h'), Key::Unicode('o')]);
	send_key(Key::Physical(Physical::Return));

	// complicated
	press_key(Key::Physical(Physical::Shift));
	send_combo(&[
		Key::Physical(Physical::E), Key::Unicode('c'), Key::Unicode('h'), Key::Unicode('o')]);
	release_key(Key::Physical(Physical::Shift));
	send_key(Key::Physical(Physical::Return));
}
