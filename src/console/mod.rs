use std::{thread, time};

/// # Wait
/// 
/// Use this function:
/// ```rs
/// console::wait(std::time::Duration);
/// ```

pub fn wait(duration: time::Duration) {
    thread::sleep(duration)
}

/// # Clear
/// 
/// Use this function:
/// ```rs
/// console::clear();
/// ```

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// # Clear after
/// 
/// Use this function:
/// ```rs
/// console::clear_after(std::time::Duration);
/// ```

pub fn clear_after(duration: time::Duration) {
    wait(duration);
    clear();
}

/// # Out
/// 
/// Use this function:
/// ```rs
/// console::out(str, bool);
/// ```

pub fn out(value: std::string::String, inline: bool) {
    if inline == true {
        println!("{}", value);
    } else {
        print!("{}", value);
    }
}
