use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::{thread,time};

use images::{Images,Results};

pub struct Server {
    id: u16,
    velocity_of_processing: f64,
    observatories : HashMap<u16,Sender<Results>>,
}

impl Server {
    pub fn new(id: u16, vel: f64, map: HashMap<u16,Sender<Results>>) -> Server {
        info!("Server created: id: {} vel: {:?}", id, vel);
        Server {
            id: id,
            velocity_of_processing : vel,
            observatories : map,
        }
    }

    pub fn run(&self, rx: Receiver<Images>) {
        let rx_ref = &rx;
        loop {
            //info!("Server {} about to receive...", self.id);
            let images = self.receive_request(rx_ref);
            info!("Server {} received {:?} from obs {}", images.srv_id, images.quads, images.obs_id);
            //sleeps velocity_of_processing secs!
            self.process_quads(images.quads);
            self.send_results(images.obs_id);
        }
    }

    fn process_quads(&self, q: u64) {
        let secs = time::Duration::from_millis(1000 * self.velocity_of_processing  as u64 * q);
        //info!("Server {} sleeping {:?}", self.id, secs);
        thread::sleep(secs);
        //info!("Server {} ends sleeping", self.id);
    }

    fn receive_request(&self, rx: &Receiver<Images>) -> Images {
        rx.recv().unwrap()
    }

    fn send_results(&self, id : u16) {
        info!("Server {} respond to obs {}", self.id, &id);
        self.observatories[&id].send(Results{found : 0, srv_id: self.id}).unwrap();
    }
}
