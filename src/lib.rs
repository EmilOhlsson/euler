extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub mod big;

pub fn answer<T: Into<String>>(s: T) {
    let ans = s.into();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(ans.clone()).unwrap();
    println!("{}", ans);
}
