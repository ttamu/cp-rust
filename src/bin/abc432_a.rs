use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let mut v = [a, b, c];
    v.sort();
    println!("{}{}{}", v[2], v[1], v[0]);
}
