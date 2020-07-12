#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: [(usize, usize, usize); n],

    }
    // |xi - xi+1| + |yi - yi+1|がt2-t1より大きく、かつ偶奇が一致していればいい
    let t0: (isize, isize, isize) = (0, 0, 0);
    let mut ans = true;
    let distance =
        (t[0].1 as isize - t0.1 as isize).abs() + (t[0].2 as isize - t0.2 as isize).abs();
    if distance > t[0].0 as isize - t0.0 || distance % 2 != (t[0].0 as isize - t0.0) % 2 {
        ans = false;
    }
    for i in 0..(n - 1) {
        let distance = (t[i + 1].1 as isize - t[i].1 as isize).abs()
            + (t[i + 1].2 as isize - t[i].2 as isize).abs();
        if distance as usize > t[i + 1].0 - t[i].0
            || distance as usize % 2 != (t[i + 1].0 - t[i].0) % 2
        {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
