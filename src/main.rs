extern crate log;

//use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
//use std::sync::{Arc, Mutex};

mod observatory;
mod images;
mod telescope;
mod server;

use observatory::Observatory;
use images::Images;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let observatory = Observatory::new(2.0, 4, 0, vec![tx1]);
    thread::spawn(move|| observatory.run(rx2));
    let dato = rx1.recv().unwrap();
    let img = Images { im : 5, obs_id: 0 };
    tx2.send(6).unwrap();
    println!("main {:?}", dato.obs_id);
}
