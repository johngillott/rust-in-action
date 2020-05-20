fn main() {
    let a = 10;
    let b: i32 = 30;

    let c = add(a, b);

    println!("a + b = {}", c)
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}