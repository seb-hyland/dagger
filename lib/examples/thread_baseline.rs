use std::{sync::mpsc, thread};

fn main() {
    let (sender, receiver) = mpsc::sync_channel(2);

    thread::scope(|s| {
        s.spawn(|| {
            let _ = sender.send("Hi!");
        });
        s.spawn(|| {
            let _ = sender.send("Other!");
        });
    });

    for _ in 0..2 {
        println!("{:#?}", receiver.recv());
    }
}
