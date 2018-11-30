extern crate log;

use std::vec;

struct Observatory {
    telescope: Telescope,
    servers: Vec<Server>,
}

impl Observatory {
    fn run(&self) {
        loop {
            let images : Images = self.telescope.take_images();
            self.send_images_to_servers(images);
            self.receive_responses_from_servers();
        }
    }
}
