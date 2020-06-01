extern crate sysinfo;
use cursive::views::Dialog;
use cursive::Cursive;
use cursive::views::LinearLayout;

use sysinfo::{NetworkExt, ProcessExt, ProcessorExt, System, SystemExt};


fn get_my_processes(system : &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();
    for (pid, process) in system.get_processes() {
        my_vec.push(pid.to_string());
        my_vec.push(process.name().to_string());
    }
    let mut my_s = String::new();
    for x in (0..my_vec.len()).step_by(2) {
        my_s.push_str(&my_vec[x].to_string());
        my_s.push_str(": ");
        my_s.push_str(&my_vec[x + 1].to_string());
        my_s.push_str("\n");
    }
    return my_s;
}


fn get_my_cpu_usage(system : &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();


    for processor in system.get_processors() {
        my_vec.push(processor.get_cpu_usage());
    }
    let mut my_s = String::new();
    for x in (0..my_vec.len()).step_by(1) {
        my_s.push_str(&format!("{:^3}", &my_vec[x]).to_string());
        my_s.push_str(" [");
        for i in (0..100).step_by(2) {
            if i < my_vec[x] as u8 {
                my_s.push_str("|");
            }
            else {
                my_s.push_str(" ");
            }
        }
        my_s.push_str("]\n");
    }
    return my_s;
}

fn my_loop(s: &mut Cursive) {
    let mut system = sysinfo::System::new_all();
    s.pop_layer();
    s.pop_layer();

    let process_string = get_my_processes(&mut system);
    let cpu_string = get_my_cpu_usage(&mut system);

    let process = Dialog::text(process_string).title("Running Processes");
    let cpu = Dialog::text(cpu_string).title("CPU Usage");

    let  layout = LinearLayout::horizontal().child(process).child(cpu);
    s.add_layer(layout);
}


fn main() {

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(' ', |s| my_loop(s));

    my_loop(&mut siv);

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
