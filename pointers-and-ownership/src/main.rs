fn plus_one(x: &int) -> int {
    *x + 1
}

fn main() {
    let y = 10i;

    println!("{:d}", plus_one(&y));
}
