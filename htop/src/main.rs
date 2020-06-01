extern crate sysinfo;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use cursive::views::StackView;

use sysinfo::{NetworkExt, ProcessExt, ProcessorExt, System, SystemExt};
use std::{thread, time};


fn get_processes(system : &mut sysinfo::System) -> std::vec::Vec<String> {
    system.refresh_all();
    let mut vec_pid = Vec::new();
    for (pid, process) in system.get_processes() {
        vec_pid.push(pid.to_string());
        vec_pid.push(process.name().to_string());
    }
    return vec_pid;
}

fn my_loop(s: &mut Cursive) {
    let mut system = sysinfo::System::new_all();
    let one_sec = time::Duration::from_millis(1000);
    let mut count = 0u32;
    s.pop_layer();

    let vec_pid = get_processes(&mut system);
    let mut my_s = String::new();
    for x in (0..vec_pid.len()).step_by(2) {
        my_s.push_str(&vec_pid[x].to_string());
        my_s.push_str(": ");
        my_s.push_str(&vec_pid[x + 1].to_string());
        my_s.push_str("\n");
    }

    s.add_layer(Dialog::text(my_s).title("Running Processes").button("Next", my_loop));

}


fn main() {

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("Press <Next> when you're ready.")
        .title("Htop")
        .button("Next", my_loop));

    siv.run();

    // siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
    // .title("Cursive")
    // .button("Quit", |s| s.quit()));

    // // Starts the event loop.
    // siv.run();


    // Now let's print every process' id and name:
    // for (pid, proc_) in system.get_processes() {
    //     println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
    // }
    
    // // Then let's print the temperature of the different components:
    // for component in system.get_components() {
    //     println!("{:?}", component);
    // }
    
    // // And then all disks' information:
    // for disk in system.get_disks() {
    //     println!("{:?}", disk);
    // }
    

    
    // println!("PROCESS");
    // for (pid, process) in system.get_processes() {
    //     println!("[{}] {} {:?} {}Kb", pid, process.name(), process.disk_usage(), process.memory());
    // }

    // println!("PROCESSOR");
    // for processor in system.get_processors() {
    //     println!("{}%", processor.get_cpu_usage());
    // }

    // println!("NETWORK");
    // for (interface_name, data) in system.get_networks() {
    //     println!("{}: {}/{} B", interface_name, data.get_received(), data.get_transmitted());
    // }

    // println!("MEMORY");
    // println!("total memory: {} KiB", system.get_total_memory());
    // println!("used memory : {} KiB", system.get_used_memory());
    // println!("total swap  : {} KiB", system.get_total_swap());
    // println!("used swap   : {} KiB", system.get_used_swap());
}
