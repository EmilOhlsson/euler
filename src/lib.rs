extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub mod big;
pub mod series;

/// Macro for printing answer to standard output, with newline, and paste it into clipboard
#[macro_export]
macro_rules! answer {
	() => (panic!("No answer given"));
	($fmt: expr) => ($crate::print_answer($fmt));
	($fmt: expr, $($arg:tt)*) => ($crate::print_answer(format!($fmt, $($arg)*)));
}

pub fn print_answer(s: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{}", s);
    ctx.set_contents(s).unwrap();
}
