extern crate libxdo;
extern crate itertools;

use std::borrow::Cow;
use self::itertools::Itertools;
use self::libxdo::XDo;
use super::{Physical, Key};

const DELAY: u32 = 12_000; // microseconds

fn get_physical_keysym(key: Physical) -> &'static str {
    use Physical::*;
    match key {
        Return => "Return",
        Shift => "Shift",
        Control => "Control",
        Alt => "Alt",
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "h",
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        M => "m",
        N => "n",
        O => "o",
        P => "p",
        Q => "q",
        R => "r",
        S => "s",
        T => "t",
        U => "u",
        V => "v",
        W => "w",
        X => "x",
        Y => "y",
        Z => "z",
    }
}

fn get_unicode_keysym(c: char) -> String {
    format!("U{:04X}", c as u32)
}

fn get_keysym(key: Key) -> Cow<'static, str> {
    match key {
        Key::Unicode(ch) => get_unicode_keysym(ch).into(),
        Key::Physical(key) => get_physical_keysym(key).into(),
    }
}

pub fn press_key(key: Key) {
    if let Ok(xdo) = XDo::new(None) {
        let sym = get_keysym(key);
        let _ = xdo.send_keysequence_down(&sym, DELAY);
    }
}

pub fn release_key(key: Key) {
    if let Ok(xdo) = XDo::new(None) {
        let sym = get_keysym(key);
        let _ = xdo.send_keysequence_up(&sym, DELAY);
    }
}

pub fn send_combo(keys: &[Key]) {
    if let Ok(xdo) = XDo::new(None) {
        let sym = keys.iter().cloned().map(get_keysym).join("+");
        let _ = xdo.send_keysequence(&sym, DELAY);
    }
}

pub fn send_key(key: Key) {
    match key {
        Key::Unicode(c) => send_char(c),
        Key::Physical(key) => if let Ok(xdo) = XDo::new(None) {
            let sym = get_physical_keysym(key);
            let _ = xdo.send_keysequence(sym, DELAY);
        }
    }
}

/// Send a unicode character. Unsupported chars are silently ignored.
pub fn send_char(c: char) {
    send_str(&c.to_string());
}

/// Send a string as keyboard events. Unsupported chars are silently ignored.
pub fn send_str(msg: &str) {
    if let Ok(xdo) = XDo::new(None) {
        let _ = xdo.enter_text(msg, DELAY);
    }
}
