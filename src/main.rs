#[macro_use]
extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
//use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod observatory;
mod images;
mod telescope;
mod server;

use observatory::Observatory;
use images::{Images,Results};
use server::Server;

const TOTAL_OBS: u16 = 1;
const TOTAL_SRV: u16 = 1;

fn main() {
    info!("New run");
    //create channels
    let (txs_res, rxs_res) = init_res_channels(TOTAL_SRV);
    let (txs_img, rxs_img) = init_img_channels(TOTAL_OBS);

    //create structs for threads
    let rxs_img_ref = &rxs_img;
    let observatories = create_observatories(txs_img.clone(), TOTAL_SRV, TOTAL_OBS);
    let servers = create_servers(txs_res.clone(), rxs_img_ref, TOTAL_SRV, TOTAL_OBS);

    //spawn threads
    let mut id = 0;
    for observatory in observatories {
        let rx = rxs_res[id];
        thread::spawn(move|| observatory.run(rx));
        id += 1;
    }
    id = 0;
    for server in servers {
        thread::spawn(move|| server.run(rxs_img[id]));
        id += 1;
    }
}

fn create_observatories(txs: Vec<Sender<Images>>,
                        total_srv: u16,
                        total_obs: u16) -> Vec<Observatory> {

    let mut observatories = Vec::new();
    for id in 0..total_obs {
        observatories.push(Observatory::new(2.0, 4, id, txs));
    }
    observatories
}

fn create_servers(txs: HashMap<u16, Sender<Results>>,
                  rxs: &Vec<Receiver<Images>>,
                  total_srv: u16,
                  total_obs: u16) -> Vec<Server> {

    let mut servers = Vec::new();
    for _ in 0..total_srv {
        servers.push(Server::new(1.0, txs));
    }
    servers
}


fn init_img_channels(total_srv: u16) -> (Vec<Sender<Images>>, Vec<Receiver<Images>>) {
    let mut txs = Vec::new();
    let mut rxs = Vec::new();
    for _ in 0..total_srv {
        let (tx, rx) = mpsc::channel();
        txs.push(tx);
        rxs.push(rx);
    }
    (txs, rxs)
}

fn init_res_channels(total_obs: u16) -> (HashMap<u16, Sender<Results>>, Vec<Receiver<Results>>) {
    let mut map = HashMap::new();
    let mut rxs = Vec::new();
    for o in 0..total_obs {
        let (tx, rx) = mpsc::channel();
        rxs.push(rx);
        map.insert(o, tx);
    }
    (map, rxs)
}
