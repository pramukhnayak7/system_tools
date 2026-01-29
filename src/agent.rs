use std::net::TcpStream;
use std::time::{Duration,SystemTime,UNIX_EPOCH};
use std::io::Write;
use sysinfo::System;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: f32,
    pub memory_mb: u64,
    pub timestamp: u64,
   // pub timestamp2: DateTime<Utc>,
}


fn main(){
    let mut system =System::new_all();
    let mut stream = TcpStream::connect("127.0.0.1:9000").expect("unable to connect");
    loop{
        system.refresh_all();
        let unix_time: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

let stats = SystemStats{
    cpu: system.global_cpu_usage(),
    memory_mb: (system.used_memory()/1024)/1024,
    timestamp: unix_time,
   // timestamp2 : DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
};

        let json = serde_json::to_string(&stats).unwrap();
        stream.write_all(json.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }
}