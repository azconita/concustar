extern crate log;

use std::vec;
use telescope::Telescope;
use images::Images;
use server::Server;
use std::sync::mpsc::{Sender, Receiver};

pub struct Observatory {
    telescope: Telescope,
    servers: Vec<u16>,
}

impl Observatory {
    pub fn new(vel: f64, quads: u16) -> Observatory {
        Observatory {
            telescope: Telescope::new(vel,quads),
            servers: vec![1, 2, 3],
        }
    }

    pub fn run(&self, tx: Sender<u16>, rx: Receiver<u16>) {
        let mut tx_ref = &tx;
        let mut rx_ref = &rx;
        loop {
            let images : Images = self.telescope.take_images();
            self.send_images_to_servers(images, tx_ref);
            self.receive_responses_from_servers(rx_ref);
        }
    }

    fn send_images_to_servers(&self, images: Images, tx: &Sender<u16>) {
        tx.send(images.im).unwrap();
    }

    fn receive_responses_from_servers(&self, rx: &Receiver<u16>) {
        let dato = rx.recv().unwrap();
        println!("{:?}", dato);
    }
}
