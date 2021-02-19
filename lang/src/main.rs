fn main() {
    let s2;
    {
        let s = "Hello, world!";
        s2 = String::from(s);
    }

    println!("{}", s2);
}
