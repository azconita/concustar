#[macro_use]
extern crate log;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread,time};
//use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod observatory;
mod images;
mod telescope;
mod server;
mod parser;

use observatory::Observatory;
use images::{Images,Results};
use server::Server;
use parser::{get_speeds, get_observatories};

//const TOTAL_OBS: u16 = 2;
//const TOTAL_SRV: u16 = 2;

fn main() {
    info!("New run");
    //read config
    let server_velocities: Vec<f64> = get_speeds();
    let observatories_config: Vec<Vec<f64>> = get_observatories();
    let TOTAL_OBS = observatories_config.len() as u16;
    let TOTAL_SRV = server_velocities.len() as u16;

    //create channels
    let (txs_res, mut rxs_res) = init_res_channels(TOTAL_OBS);
    let (txs_img, mut rxs_img) = init_img_channels(TOTAL_SRV);

    //create structs for threads
    let observatories = create_observatories(txs_img, TOTAL_OBS, observatories_config);
    let servers = create_servers(txs_res, TOTAL_SRV, server_velocities);

    //spawn threads
    let mut id = 0;
    for observatory in observatories {
        let rx = rxs_res.pop().unwrap();
        thread::spawn(move|| observatory.run(rx));
        id += 1;
    }
    id = 0;
    for server in servers {
        let rx = rxs_img.pop().unwrap();
        thread::spawn(move|| server.run(rx));
        id += 1;
    }
    let ten_secs = time::Duration::from_secs(5);
    thread::sleep(ten_secs);
}

fn create_observatories(txs: Vec<Sender<Images>>,
                        total_obs: u16,
                        observatories_config: Vec<Vec<f64>>) -> Vec<Observatory> {

    let mut observatories = Vec::with_capacity(total_obs as usize);
    for id in 0..total_obs {
        let mut config = observatories_config[id as usize].clone();
        observatories.push(Observatory::new(config, id, txs.clone()));
    }
    observatories
}

fn create_servers(txs: HashMap<u16, Sender<Results>>,
                  total_srv: u16,
                  server_velocities: Vec<f64>) -> Vec<Server> {

    let mut servers = Vec::with_capacity(total_srv as usize);
    for i in 0..total_srv {
        servers.push(Server::new(i, server_velocities[i as usize], txs.clone()));
    }
    servers
}


fn init_img_channels(total_srv: u16) -> (Vec<Sender<Images>>, Vec<Receiver<Images>>) {
    let mut txs = Vec::with_capacity(total_srv as usize);
    let mut rxs = Vec::with_capacity(total_srv as usize);
    for _ in 0..total_srv {
        let (tx, rx) = mpsc::channel();
        txs.push(tx);
        rxs.push(rx);
    }
    (txs, rxs)
}

fn init_res_channels(total_obs: u16) -> (HashMap<u16, Sender<Results>>, Vec<Receiver<Results>>) {
    let mut map = HashMap::new();
    let mut rxs = Vec::with_capacity(total_obs as usize);
    for o in 0..total_obs {
        let (tx, rx) = mpsc::channel();
        rxs.push(rx);
        map.insert(o, tx);
    }
    (map, rxs)
}
