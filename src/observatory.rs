extern crate log;

use std::time::Instant;
use telescope::Telescope;
use images::{Images,Results};
//use server::Server;
use std::sync::mpsc::{Sender, Receiver};

pub struct Observatory {
    id: u16,
    telescope: Telescope,
    servers: Vec<Sender<Images>>,
}

impl Observatory {
    pub fn new(mut config: Vec<f64>, id: u16, srvs: Vec<Sender<Images>>) -> Observatory {
        let v = config.remove(0);
        let q = config.remove(0);
        info!("Observatory created: id {} servers_config: {:?}", id, config);
        Observatory {
            id: id,
            telescope: Telescope::new(v, q as u16, id, config),
            servers: srvs,
        }
    }

    pub fn run(&self, rx: Receiver<Results>) {
        let rx_ref = &rx;
        loop {
            let images : Vec<Images> = self.telescope.take_images();
            let now = Instant::now();
            self.send_images_to_servers(images);
            self.receive_responses_from_servers(rx_ref);
            info!("Obs {}: Request time {:?}", self.id, now.elapsed());
        }
    }

    fn send_images_to_servers(&self, images: Vec<Images>) {
        for image in images {
            let tx = self.servers[image.srv_id as usize].clone();
            info!("Obs {} about to send...", self.id);
            tx.send(image).unwrap();
            info!("Obs {} sent", self.id);
        }
    }

    fn receive_responses_from_servers(&self, rx: &Receiver<Results>) {
        let dato = rx.recv().unwrap();
        info!("dato  {:?}", dato.found);
    }
}
