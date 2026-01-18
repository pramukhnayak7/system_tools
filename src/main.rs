use sysinfo::System;
use std::{thread, time::Duration};

fn main() {
    let mut system =System::new_all();

    loop{

    system.refresh_processes();
    print!("\x1B[2J\x1B[1;1H");



    println!("{:<10} {:<20} {:>5} {:>5}","PID","NAME","CPU%","MEM(MB)");

    for(pid,process) in system.processes(){
        let memory_mb = process.memory() as f64/1024.0;
        println!("{:<10} {:<20} {:>5} {:>5}",pid,process.name(),process.cpu_usage(),memory_mb);
    }

    thread::sleep(Duration::from_secs(1));

}
}
