/*
 * Rustの関数。
 * CreatedAt: 2019-05-31
 */
fn main() {
    let x = 1;
//    let y = (let z = 2);
    let y = { x + 1 };
    println!("{} {}", x, y);
//    let y = { x + 1; };
//    println!("{} {:?}", x, y);
}

