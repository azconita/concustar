use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::{thread,time};

use images::{Images,Results};

pub struct Server {
    id: u16,
    velocity_of_processing: f64,
    observatorys : HashMap<u16,Sender<Results>>,
}

impl Server {
    pub fn new(id: u16, vel: f64, map: HashMap<u16,Sender<Results>>) -> Server {
        println!("Server created: id: {} vel: {:?}", id, vel);
        Server {
            id: id,
            velocity_of_processing : vel,
            observatorys : map,
        }
    }

    pub fn run(&self, rx: Receiver<Images>) {
        let mut rx_ref = &rx;
        loop {
            println!("Server {} about to receive...", self.id);
            let images = self.receive_request(rx_ref);
            println!("Server {} received {:?}", self.id, images.quads);
            //sleeps velocity_of_processing secs!
            self.process_quads(images.quads);
            self.send_results(images.obs_id);
        }
    }

    fn process_quads(&self, q: u64) {
        let secs = time::Duration::from_millis(1000 * self.velocity_of_processing  as u64 * q);
        println!("Server {} sleeping {:?}", self.id, secs);
        thread::sleep(secs);
    }

    fn receive_request(&self, rx: &Receiver<Images>) -> Images {
        rx.recv().unwrap()
    }

    fn send_results(&self, id : u16) {
        self.observatorys[&id].send(Results{found : 0}).unwrap();
    }
}
