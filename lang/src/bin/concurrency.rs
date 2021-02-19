use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    // case1();
    // case2();
    // case3();
    // case4();
    // case5();
    case6();
}

fn case6() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
           let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    };

    for handle in handles {
        handle.join().unwrap()
    }

    println!("result is {}", *counter.lock().unwrap());
}

fn case5() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 1;
    }

    println!("m = {:?}", m);
}

fn case4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move|| {
       let vals = vec![
           String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),
       ];

       for val in vals {
           tx.send(val).unwrap();
           thread::sleep(Duration::from_millis(1));
       }

    });

    for val in rx {
        println!("Got: {}", val);
    }
}

fn case3() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn case2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here is vector: {:?}.", v);
    });

    handle.join().unwrap();
}

fn case1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}