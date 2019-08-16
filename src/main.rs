use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Ground control to Major Tom...");
        println!("Sending: {}", &val);
        
        // send takes ownership of val
        let sender = tx.send(val);
        match sender {
            Ok(val) => val,
            Err(e) => panic!("Could not send msg: {:?}", e)
        };
    });

    // received now has ownership of the value sent
    let received = rx.recv().unwrap();

    println!("Received: {}", received);
}
