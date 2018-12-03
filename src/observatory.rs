extern crate log;

//use std::vec;
use telescope::Telescope;
use images::{Images,Results};
//use server::Server;
use std::sync::mpsc::{Sender, Receiver};

pub struct Observatory {
    telescope: Telescope,
    servers: Vec<Sender<Images>>,
}

impl Observatory {
    pub fn new(vel: f64, quads: u16, id: u16, srvs: Vec<Sender<Images>>) -> Observatory {
        Observatory {
            telescope: Telescope::new(vel, quads, id),
            servers: srvs,
        }
    }

    pub fn run(&self, rx: Receiver<Results>) {
        let rx_ref = &rx;
        loop {
            let images : Images = self.telescope.take_images();
            self.send_images_to_servers(images);
            self.receive_responses_from_servers(rx_ref);
        }
    }

    fn send_images_to_servers(&self, images: Images) {
        for tx in self.servers.clone() {
            let i = Images { im: images.im, obs_id: images.obs_id };
            tx.send(i).unwrap();
        }
    }

    fn receive_responses_from_servers(&self, rx: &Receiver<Results>) {
        let dato = rx.recv().unwrap();
        println!("dato  {:?}", dato.found);
    }
}
