use proconio::{input, marker::Chars};

fn main() {
    input! {c: Chars}

    let rle = {
        let mut res = vec![];
        let mut l = 0;
        while l < c.len() {
            let mut r = l;
            while r + 1 < c.len() && c[l] == c[r + 1] {
                r += 1;
            }
            res.push((c[l].to_digit(10).unwrap(), r - l + 1));
            l = r + 1;
        }
        res
    };

    let mut ans = 0;
    for i in 0..rle.len() - 1 {
        if rle[i].0 + 1 == rle[i + 1].0 {
            ans += std::cmp::min(rle[i].1, rle[i + 1].1);
        }
    }
    println!("{}", ans);
}
