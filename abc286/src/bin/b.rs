use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    };

    let ans = s.replace("na", "nya");
    println!("{}", ans);
}
