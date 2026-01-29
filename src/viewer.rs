use std::net::TcpListener;
use std::io::{BufRead,BufReader};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: f32,
    pub memory_mb: u64,
    pub timestamp: u64,
}

fn main(){
    let listener = TcpListener::bind("0.0.0.0:9000").expect("unable to connect");

    let (stream,addr)=listener.accept().unwrap();

    let reader = BufReader::new(stream);

    for line in reader.lines(){
        let line = line.unwrap();
        let stats: SystemStats= serde_json::from_str(&line).unwrap();
          println!(
            "CPU: {:>5.1}% | MEM: {:>6} MB | TIME: {}",
            stats.cpu,
            stats.memory_mb,
            stats.timestamp
        );
    }
}
