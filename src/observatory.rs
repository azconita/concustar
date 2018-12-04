extern crate log;

//use std::vec;
use telescope::Telescope;
use images::{Images,Results};
//use server::Server;
use std::sync::mpsc::{Sender, Receiver};

pub struct Observatory {
    id: u16,
    telescope: Telescope,
    servers: Vec<Sender<Images>>,
    servers_config: Vec<f64>,
}

impl Observatory {
    //pub fn new(vel: f64, quads: u16, id: u16, srvs: Vec<Sender<Images>>) -> Observatory {
    pub fn new(mut config: Vec<f64>, id: u16, srvs: Vec<Sender<Images>>) -> Observatory {
        let v = config.remove(0);
        let q = config.remove(0);
        Observatory {
            id: id,
            telescope: Telescope::new(v, q as u16, id),
            servers: srvs,
            servers_config: config,
        }
    }

    pub fn run(&self, rx: Receiver<Results>) {
        let rx_ref = &rx;
        //loop {
        //    let images : Images = self.telescope.take_images();
        //    self.send_images_to_servers(images);
        //    self.receive_responses_from_servers(rx_ref);
        //}
        println!("{} {:?}", self.id, self.servers_config);
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
