extern crate log;

use std::time::Instant;
use images::Results;
//use server::Server;
use std::sync::mpsc::Receiver;

pub struct Observatory {
    id: u16,
    servers: Vec<u64>,
    average_time_per_request: f64,
    total_requests: u64,
    total_request_time: f64,
}

impl Observatory {
    pub fn new(id: u16, n_servers: usize) -> Observatory {
        info!("Observatory created: id {}", id);
        Observatory {
            id: id,
            servers: Vec::with_capacity(n_servers as usize),
            average_time_per_request : 0 as f64,
            total_requests: 0 as u64,
            total_request_time: 0 as f64,
        }
    }

    pub fn run(mut self, rx: Receiver<Results>, n_servers: u16) {
        for _ in 0..n_servers {
            self.servers.push(0 as u64);
        }
        let rx_ref = &rx;
        let mut now = Instant::now();
        loop {
            self.receive_responses_from_servers(rx_ref);
            let mut request_acomplished = 1;
            for i in 0..self.servers.len() {
                if self.servers[i] == 0 {
                    request_acomplished = 0;
                }
            }
            if request_acomplished == 1 {
                let request_time = now.elapsed().subsec_millis() as f64;
                for i in 0..self.servers.len() {
                    self.servers[i] -= 1;
                }
                now = Instant::now();
                self.total_requests += 1;
                self.total_request_time += request_time;
                self.average_time_per_request = self.total_request_time / self.total_requests as f64;
                error!("Obs {}: req time: {}, average: {:?}, total reqs: {}, total time: {}", self.id, request_time, self.average_time_per_request, self.total_requests, self.total_request_time);
            }
        }
    }

    fn receive_responses_from_servers(&mut self, rx: &Receiver<Results>) {
            let dato = rx.recv().unwrap();
            info!("Obs {}: received from {} dato  {:?}", self.id, dato.srv_id, dato.found);
            self.servers[dato.srv_id as usize] += 1;
    }
}
