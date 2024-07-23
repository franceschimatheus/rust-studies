use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
}
