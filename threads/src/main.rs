use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("i"),
            String::from("love"),
            String::from("HARDCORE"),
            String::from("VECHORNYTSI"),
        ];
        for message in messages {
            tx.send(message);
            thread::sleep(Duration::from_secs(2));
        }
    });

    for (j,received_messages) in rx.iter().enumerate(){
        println!("{}", received_messages)
    }


}
