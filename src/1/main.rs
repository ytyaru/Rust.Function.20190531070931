/*
 * Rustの関数。
 * CreatedAt: 2019-05-31
 */
fn main() {
    show_age(66);
    show_coordinate(12, 34);
}
//fn show_age(age) {
fn show_age(age: i32) {
    println!("My age is {}.", age);
}
fn show_coordinate(x:i32, y:i32) {
    println!("({}, {})", x, y);
}
