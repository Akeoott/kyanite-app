// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.use std::{thread, time::Duration};

use kyanite_mov_lib::{cpu_snapshot, drive_snapshot, gpu_snapshot, memory_snapshot, network_snapshot, system_snapshot, update_all};

fn main() {
    update_all();

    let ten_millis = Duration::from_millis(1000);
    thread::sleep(ten_millis);

    update_all();

    let cpu = cpu_snapshot();
    let mem = memory_snapshot();
    let gpu = gpu_snapshot();
    let drive = drive_snapshot();
    // let proc = process_snapshot();
    let sys = system_snapshot();
    let net = network_snapshot();

    println!("{}", cpu);
    println!("{}", mem);
    println!("{}", gpu);
    println!("{}", drive);
    // println!("{}", proc);
    println!("{}", sys);
    println!("{}", net);
    println!("process_snapshot is hidden due to massive output");
}
