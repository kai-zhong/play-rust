fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    println!("{}, {}, {}", s1, s2, s);

    let mut s3 = &s;

    println!("{}, {}, {}", s1, s2, s3);

    println!("char is {}.", s.chars().nth(3).unwrap());
    s.clear();

}