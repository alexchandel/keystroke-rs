# keystroke-rs
Send a string, character, or keystroke event to the system.

Contributions welcome!

## Examples

```rust
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
		Key::Physical(Physical::E), Key::Unicode('c'), Key::Unicode('h'), Key::Unicode('o'),
		Key::Physical(Physical::Return)]);
	send_key(Key::Physical(Physical::Return));

	// complicated
	use Key::{Physical, Unicode};
	use Physical::{E, C, H, O, Return, Shift};
	press_key(Physical(Shift));
	send_combo(&[
		Physical(E), Physical(C), Physical(H), Physical(O)]);
	release_key(Physical(Shift));
	send_key(Physical(Return));
}
```

TLDR; call `keystroke::send_str` with a `&str`.

## Development

* Only supports Windows and Linux right now. PRs adding MacOS and other OS are welcome!
