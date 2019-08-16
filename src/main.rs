use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("clone"),
        ];

        for val in vals {
            let outbound = tx1.send(val);
            match outbound {
                Ok(val) => val,
                Err(e) => panic!("Could not send msg: {:?}", e),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("this"),
            String::from("message"),
            String::from("from"),
            String::from("original"),
        ];

        for val in vals {
            let outbound = tx.send(val);
            match outbound {
                Ok(val) => val,
                Err(e) => panic!("Could not send msg: {:?}", e),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
