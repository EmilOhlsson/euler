#[macro_use]
extern crate project_euler;

fn main() {
    for a in 1..999 {
        for b in (a + 1)..999 {
            for c in (b + 1)..999 {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    answer!("{}", a * b * c);
                    return;
                }
            }
        }
    }
    panic!("No answer found");
}
