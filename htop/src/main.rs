extern crate sysinfo;

use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::Cursive;

use sysinfo::{DiskExt, ProcessExt, ProcessorExt, SystemExt};

use std::{thread, time};

fn get_my_processes(system: &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();
    for (pid, process) in system.get_processes() {
        my_vec.push(pid.to_string());
        my_vec.push(process.name().to_string());
        my_vec.push(format!("{:?}", process.cpu_usage()));
        my_vec.push(format!("{:?}", process.memory()));
    }
    let mut my_s = String::with_capacity(2048);
    my_s.push_str(&format!(
        "{:^5}: {:^6}: {:^6}: {:^6}\n",
        "Pid", "Name", "Cpu(%)", "Memory(kb)"
    ));
    for x in (0..my_vec.len()).step_by(4) {
        my_s.push_str(&format!("{:^5}", &my_vec[x]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^6}", &my_vec[x + 1]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^6}", &my_vec[x + 2])[0..6]);
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^10}", &my_vec[x + 3]));
        my_s.push_str("\n");
    }
    return my_s;
}

fn get_my_cpu_usage(system: &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();

    for processor in system.get_processors() {
        my_vec.push(processor.get_cpu_usage());
    }
    let mut my_s = String::with_capacity(2048);
    for x in (0..my_vec.len()).step_by(1) {
        my_s.push_str(&format!("[{:^2}]", x));
        my_s.push_str(" [");
        for i in (0..100).step_by(2) {
            if i < my_vec[x] as u8 {
                my_s.push_str("|");
            } else {
                my_s.push_str(" ");
            }
        }
        my_s.push_str("] ");
        my_s.push_str(&format!("{:^3}", &my_vec[x]));
        my_s.push_str("\n");
    }
    return my_s;
}

fn get_disk_type_string(disk: sysinfo::DiskType) -> String {
    match disk {
        sysinfo::DiskType::HDD => String::from("HDD"),
        sysinfo::DiskType::SSD => String::from("SSD"),
        _ => String::from("Unknown"),
    }
}

fn get_my_disks(system: &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();
    for disk in system.get_disks() {
        my_vec.push(disk.get_name().to_string_lossy().into_owned());
        my_vec.push(get_disk_type_string(disk.get_type()));
        my_vec.push(format!("{:?}", disk.get_mount_point()));
        my_vec.push(format!("{:?}", disk.get_total_space() / 1000000000));
        my_vec.push(format!("{:?}", disk.get_available_space() / 1000000000));
    }
    let mut my_s = String::with_capacity(2048);

    my_s.push_str(&format!(
        "{:^8}: {:^8}: {:^10}: {:^12}: {:^12}\n",
        "Name", "Type", "Mount", "Total(Gb)", "Free(Gb)"
    ));
    for x in (0..my_vec.len()).step_by(5) {
        my_s.push_str(&format!("{:^8}", &my_vec[x]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^8}", &my_vec[x + 1]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^10}", &my_vec[x + 2]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^12}", &my_vec[x + 3]));
        my_s.push_str(": ");
        my_s.push_str(&format!("{:^12}", &my_vec[x + 4]));
        my_s.push_str("\n");
    }
    return my_s;
}

// println!("MEMORY");
// println!("total memory: {} KiB", system.get_total_memory());
// println!("used memory : {} KiB", system.get_used_memory());
// println!("total swap  : {} KiB", system.get_total_swap());
// println!("used swap   : {} KiB", system.get_used_swap());

fn get_my_memory(system: &mut sysinfo::System) -> String {
    system.refresh_all();
    let mut my_vec = Vec::new();

    my_vec.push(format!("{}", system.get_total_memory() / 1049000));
    my_vec.push(format!("{}", system.get_used_memory() / 1049000));
    my_vec.push(format!("{}", system.get_total_swap() / 1049000));
    my_vec.push(format!("{}", system.get_used_swap() / 1049000));

    let mut my_s = String::with_capacity(2048);

    my_s.push_str(&format!(
        "{:^5}: {:^5}: {:^5}: {:^5}\n",
        "Total", "Used", "Swap", "Used"
    ));
    my_s.push_str(&format!("{}(GiB)", &my_vec[0]));
    my_s.push_str(": ");
    my_s.push_str(&format!("{}(GiB)", &my_vec[1]));
    my_s.push_str(": ");
    my_s.push_str(&format!("{}(GiB)", &my_vec[2]));
    my_s.push_str(": ");
    my_s.push_str(&format!("{}(GiB)", &my_vec[3]));
    my_s.push_str("\n");
    return my_s;
}

fn my_loop(s: &mut Cursive) {
    let mut system = sysinfo::System::new_all();
    s.pop_layer();

    let process_string = get_my_processes(&mut system);
    let cpu_string = get_my_cpu_usage(&mut system);
    let disk_string = get_my_disks(&mut system);
    let memory_string = get_my_memory(&mut system);

    let process = Dialog::text(process_string).title("Running Processes");
    let cpu = Dialog::text(cpu_string).title("CPU Usage");
    let disks = Dialog::text(disk_string).title("Disks Info");
    let memory = Dialog::text(memory_string).title("Memory Info");

    let layout = LinearLayout::vertical()
        // .child(LinearLayout::horizontal()
        .child(cpu)
        .child(memory)
        .child(LinearLayout::horizontal().child(process).child(disks));
    s.add_layer(layout);
}

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    while siv.is_running() {
        my_loop(&mut siv);
        siv.step();
        siv.refresh();
        thread::sleep(time::Duration::from_millis(1000));
    }

    // println!("NETWORK");
    // for (interface_name, data) in system.get_networks() {
    //     println!("{}: {}/{} B", interface_name, data.get_received(), data.get_transmitted());
    // }
}
