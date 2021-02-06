fn main() {

    let s = Test{
        v: 32
    };


    let v = s.unwrap();

    println!("s is {}", s.v);
    println!("v is {}", v);

}

#[derive(Copy, Clone)]
struct Test {
    v: i32
}

impl Test {
    fn unwrap(self) -> i32 {
        self.v
    }
}
