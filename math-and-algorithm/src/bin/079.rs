use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!("{}", (n - 1) * n / 2);
}
