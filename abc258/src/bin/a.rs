use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let h = 21 + k / 60;
    let m = k % 60;

    println!("{}:{:0width$}", h, m, width = 2);
}
