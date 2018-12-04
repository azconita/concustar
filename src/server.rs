use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};

use images::{Images,Results};

pub struct Server {
    id: u16,
    velocity_of_processing: f64,
    observatorys : HashMap<u16,Sender<Results>>,
}

impl Server {
    pub fn new(id: u16, vel: f64, map: HashMap<u16,Sender<Results>>) -> Server {
        Server {
            id: id,
            velocity_of_processing : vel,
            observatorys : map,
        }
    }

    pub fn run(&self, rx: Receiver<Images>) {
        let mut rx_ref = &rx;
        //loop {
        //    let images = self.receive_request(rx_ref);
        //    println!("{:?}", images.im);
        //    self.send_results(images.obs_id);
        //}
        println!("{} {:?}", self.id, self.velocity_of_processing);
    }

    fn receive_request(&self, rx: &Receiver<Images>) -> Images {
        rx.recv().unwrap()
    }

    fn send_results(&self, id : u16) {
        self.observatorys[&id].send(Results{found : 0}).unwrap();
    }
}
