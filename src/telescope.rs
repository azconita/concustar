extern crate log;

use std::{thread,time};
use images::Images;

pub struct Telescope {
    velocity_of_shooting: f64,
    number_of_quadrants: u16,
    quads_division: Vec<f64>,
    id: u16,
}

impl Telescope {
    pub fn new(vel: f64, quads: u16, id: u16, quads_division: Vec<f64>) -> Telescope {
        info!("Telescope created: id {} vel: {} quads: {}", id, vel, quads);

        Telescope {
            velocity_of_shooting: vel,
            number_of_quadrants: quads,
            quads_division: quads_division,
            id : id,
        }
    }
    pub fn take_images(&self) -> Vec<Images> {
        let mut v = Vec::with_capacity(self.number_of_quadrants as usize);
        let mut id = 0;
        //sleep velocity_of_shooting secs!
        let secs = time::Duration::from_millis(1000*self.velocity_of_shooting as u64);
        info!("Telescope {} sleeping {:?}", self.id, secs);
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
}
