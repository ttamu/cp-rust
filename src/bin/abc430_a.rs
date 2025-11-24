use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    println!("{}", if c >= a && d < b { "Yes" } else { "No" });
}
