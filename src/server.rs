use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};

use images::Images;

pub struct Server {
    velocity_of_processing: f64,
    observatorys : HashMap<u16,Sender<u16>>,
}

impl Server {
    pub fn new(vel: f64) -> Server {
        Server {
            velocity_of_processing : vel,
            observatorys : HashMap::new(),
        }
    }

    pub fn run(&self, tx: Sender<u16>, rx: Receiver<Images>) {
        let mut rx_ref = &rx;
        loop {
            let images = self.receive_request(rx_ref);
            self.send_results(images.obs_id);
        }
    }

    fn receive_request(&self, rx: &Receiver<Images>) -> Images {
        rx.recv().unwrap()
    }

    fn send_results(&self, id : u16) {
        self.observatorys[&id].send(9).unwrap();
    }
}
