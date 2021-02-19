fn main() {
    let mut result;

    result = longest("abc", "abcd");
    println!("the longest str is '{}'.", result);

    let a = "abc";
    let b = "bac";

    result = longest(a, b);
    println!("the longest str is '{}'.", result);

    let result2;
    {


        let c = "c";
        result = longest(a, c);
        println!("the longest str is '{}'.", result);

        result2 = longest(a, c);

    }

    println!("the longest v2 str is '{}'.", result2);


}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}