extern crate log;

use images::Images;

pub struct Telescope {
    velocity_of_shooting: f64,
    number_of_quadrants: u16,
    id: u16,
}

impl Telescope {
    pub fn new(vel: f64, quads: u16, id: u16) -> Telescope {
        Telescope {
            velocity_of_shooting: vel,
            number_of_quadrants: quads,
            id : id,
        }
    }
    pub fn take_images(&self) -> Images {
        Images {
            im : 5,
            obs_id : self.id,
        }
    }
}
