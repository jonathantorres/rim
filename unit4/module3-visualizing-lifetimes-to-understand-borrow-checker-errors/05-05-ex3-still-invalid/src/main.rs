fn main() {
    let list_a = vec![100, 34, 72, 55];
    let first_two = &list_a[0..2];
    let list_b = list_a;
    println!("First two are {:?}", first_two);
}
