extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

mod observatory;
mod images;
mod telescope;
mod server;

use observatory::Observatory;

fn main() {
    let observatory = Observatory::new(2.0, 4);
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move|| observatory.run(tx1, rx2));
    let dato = rx1.recv().unwrap();
    tx2.send(2).unwrap();
    println!("main {:?}", dato);
}
