extern crate log;

//use std::vec;
use telescope::Telescope;
use images::Images;
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

    pub fn run(&self, rx: Receiver<u16>) {
        let mut rx_ref = &rx;
        loop {
            let images : Images = self.telescope.take_images();
            self.send_images_to_servers(images);
            self.receive_responses_from_servers(rx_ref);
        }
    }

    fn send_images_to_servers(&self, images: Images) {
        let i = images;
        for tx in self.servers {
            tx.send(i).unwrap();
        }
    }

    fn receive_responses_from_servers(&self, rx: &Receiver<u16>) {
        let dato = rx.recv().unwrap();
        println!("{:?}", dato);
    }
}
