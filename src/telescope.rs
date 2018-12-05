extern crate log;

use std::{thread,time};
use std::sync::mpsc::Sender;
use images::Images;

pub struct Telescope {
    velocity_of_shooting: f64,
    quads_division: Vec<f64>,
    id: u16,
    servers: Vec<Sender<Images>>,
}

impl Telescope {
    pub fn new(vel: f64, quads: u16, id: u16, quads_division: Vec<f64>, servers: Vec<Sender<Images>>) -> Telescope {
        info!("Telescope created: id {} vel: {} quads: {}", id, vel, quads);

        Telescope {
            velocity_of_shooting: vel,
            quads_division: quads_division,
            id : id,
            servers: servers,

        }
    }
    pub fn take_images(&self) -> Vec<Images> {
        let mut v = Vec::with_capacity(self.quads_division.len() as usize);
        let mut id = 0;
        //sleep velocity_of_shooting secs!
        let secs = time::Duration::from_millis(1000*self.velocity_of_shooting as u64);
        //info!("Telescope {} sleeping {:?}", self.id, secs);
        thread::sleep(secs);
        for total_quads in self.quads_division.clone() {
            v.push(Images {
                srv_id : id,
                quads : total_quads as u64,
                obs_id : self.id,
            });
            id += 1;
        }
        v
    }

    pub fn send_images_to_servers(&self) {
        loop{
            let images : Vec<Images> = self.take_images();
            for image in images {
                let tx = self.servers[image.srv_id as usize].clone();
                //info!("Obs {} about to send...", self.id);
                info!("Obs {} sent {} to server {}", self.id, image.quads, image.srv_id);
                tx.send(image).unwrap();
            }
        }
    }

}
