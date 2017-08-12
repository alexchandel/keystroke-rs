extern crate winapi;
extern crate user32 as user32_sys;

use std::mem::{size_of, transmute_copy};
use self::winapi::{c_int, WORD};
use self::winapi::{INPUT_KEYBOARD, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, KEYEVENTF_SCANCODE};
use self::winapi::{INPUT, KEYBDINPUT};
use self::user32_sys::SendInput;

use super::{Physical, Key};

fn get_scancode(p: Physical) -> WORD {
    use Physical::*;
    match p {
        Return => 0x1c as WORD,
        Shift => 0x2a as WORD,
        Control => 0x1d as WORD,
        Alt => 0x38 as WORD,
        A => 0x1e as WORD,
        B => 0x30 as WORD,
        C => 0x2e as WORD,
        D => 0x20 as WORD,
        E => 0x12 as WORD,
        F => 0x21 as WORD,
        G => 0x22 as WORD,
        H => 0x23 as WORD,
        I => 0x17 as WORD,
        J => 0x24 as WORD,
        K => 0x25 as WORD,
        L => 0x26 as WORD,
        M => 0x32 as WORD,
        N => 0x31 as WORD,
        O => 0x18 as WORD,
        P => 0x19 as WORD,
        Q => 0x10 as WORD,
        R => 0x13 as WORD,
        S => 0x1f as WORD,
        T => 0x14 as WORD,
        U => 0x16 as WORD,
        V => 0x2f as WORD,
        W => 0x11 as WORD,
        X => 0x2d as WORD,
        Y => 0x15 as WORD,
        Z => 0x2c as WORD,
    }
}

unsafe fn key_to_lpinput(key: &Key, up: bool) -> INPUT {
    let upflag = if up { KEYEVENTF_KEYUP } else { 0 };

    match *key {
        Key::Physical(p) => {
            INPUT {
                type_: INPUT_KEYBOARD,
                u: transmute_copy(&KEYBDINPUT {
                    wVk: 0,
                    wScan: get_scancode(p), // hardware scan code
                    dwFlags: KEYEVENTF_SCANCODE | upflag,
                    time: 0,
                    dwExtraInfo: 0,
                }),
            }
        },
        Key::Unicode(c) => {
            INPUT {
                type_: INPUT_KEYBOARD,
                u: transmute_copy(&KEYBDINPUT {
                    wVk: 0,
                    wScan: c as WORD, // a unicode code
                    dwFlags: KEYEVENTF_UNICODE | upflag,
                    time: 0,
                    dwExtraInfo: 0,
                }),
            }
        },
    }
}

fn send_input(keys: &[Key], up: bool) {
    unsafe { 
        //convert all the keys to windows events
        let mut inputs: Vec<INPUT> = keys.iter().map(|k| key_to_lpinput(k, up)).collect();
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as c_int
        );
    }
}

pub fn press_key(k: Key) {
    send_input(&[k], false);
}

pub fn release_key(k: Key) {
    send_input(&[k], true);
}

pub fn send_combo(keys: &[Key]) {
    send_input(keys, false);
    send_input(keys, true);
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

/// Send a string as keyboard events. Unsupported chars are silently ignored.
pub fn send_str(msg: &str) {
    for c in msg.chars() {
        send_char(c);
    }
}
