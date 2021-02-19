use futures::executor::block_on;

async fn do_something() {
    println!("this is async fn.");
}

fn main() {
    let f = do_something();
    block_on(f);
}