/*
 * Rustの関数。
 * CreatedAt: 2019-05-31
 */
fn main() {
    let x = add_one(9);
    println!("{}", x);
}
fn add_one(x: i32) -> i32 {
    x + 1
//    x + 1; // error[E0308]: mismatched types
//    return x + 1;
}

