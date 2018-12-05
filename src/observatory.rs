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
    average_time_per_request: f64,
    total_requests: u64,
    total_request_time: f64,
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
            average_time_per_request : 0 as f64,
            total_requests: 0 as u64,
            total_request_time: 0 as f64,
        }
    }

    pub fn run(mut self, rx: Receiver<Results>) {
        let rx_ref = &rx;
        loop {
            let images : Vec<Images> = self.telescope.take_images();
            let now = Instant::now();
            self.send_images_to_servers(images);
            self.receive_responses_from_servers(rx_ref);
            let request_time = now.elapsed().subsec_millis() as f64;
            self.total_requests += 1;
            self.total_request_time += request_time;
            self.average_time_per_request = (self.average_time_per_request + request_time) / self.total_requests as f64;
            //if self.total_requests % 10 == 0 {
            println!("Obs {}: req time: {}, average: {:?}, total reqs: {}, total time: {}", self.id, request_time, self.average_time_per_request, self.total_requests, self.total_request_time);
            //}
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
        for _ in 0..self.servers.len() {
            let dato = rx.recv().unwrap();
            info!("Obs {}: received from {} dato  {:?}", self.id, dato.srv_id, dato.found);
        }
    }
}
