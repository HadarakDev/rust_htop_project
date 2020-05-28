extern crate sysinfo;
use cursive::views::TextView;

use sysinfo::{NetworkExt, ProcessExt, ProcessorExt, System, SystemExt};
use std::{thread, time};


fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
    siv.run();

    let one_sec = time::Duration::from_millis(1000);
    let mut count = 0u32;
    loop{
        count += 1;

        print!("{}[2J", 27 as char);
        let mut system = sysinfo::System::new_all();

        system.refresh_all();

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
        

        
        println!("PROCESS");
        for (pid, process) in system.get_processes() {
            println!("[{}] {} {:?} {}Kb", pid, process.name(), process.disk_usage(), process.memory());
        }

        println!("PROCESSOR");
        for processor in system.get_processors() {
            println!("{}%", processor.get_cpu_usage());
        }

        println!("NETWORK");
        for (interface_name, data) in system.get_networks() {
            println!("{}: {}/{} B", interface_name, data.get_received(), data.get_transmitted());
        }

        println!("MEMORY");
        println!("total memory: {} KiB", system.get_total_memory());
        println!("used memory : {} KiB", system.get_used_memory());
        println!("total swap  : {} KiB", system.get_total_swap());
        println!("used swap   : {} KiB", system.get_used_swap());


        thread::sleep(one_sec);
        if count == 10 {
            break;
        }
    }
}
