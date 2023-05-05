

extern crate cursive;
extern crate cursive_table_view;
extern crate rand;

use std::cmp::Ordering;


use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::*;
use sysinfo::{System,SystemExt, CpuExt};

use procfs::process::*;
use cursive::direction::Orientation;

use pad::{PadStr, Alignment};

use cursive_table_view::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name, // Process Name
    PID, // Process ID
    PPID, // Parent Process ID
    State, // Process State
    Priority, // Process Priority
    Niceness, // Process Niceness
    StartTime, // Process Start Time
    VSize, // Virtual Memory Size
    RSS, // Resident Set Size
    Threads, // Number of Threads
    CpuTime, // CPU Usage
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::PID => "PID",
            BasicColumn::PPID => "PPID",
            BasicColumn::State => "State",
            BasicColumn::Priority => "Priority",
            BasicColumn::Niceness => "Niceness",
            BasicColumn::StartTime => "Start Time",
            BasicColumn::VSize => "VSize",
            BasicColumn::RSS => "RSS",
            BasicColumn::Threads => "Threads",
            BasicColumn::CpuTime => "CPU Time",
        }
    }
}

#[derive(Clone, Debug)]
struct TableProcess {
    name: String,
    pid: i32,
    ppid: i32,
    state: char,
    priority: i64,
    niceness: i64,
    start_time: u64,
    vsize: u64,
    rss: u64,
    threads: i64,
    cpu_time: i32,
}

impl TableViewItem<BasicColumn> for TableProcess {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::PID => format!("{}", self.pid),
            BasicColumn::PPID => format!("{}", self.ppid),
            BasicColumn::State => format!("{}", self.state),
            BasicColumn::Priority => format!("{}", self.priority),
            BasicColumn::Niceness => format!("{}", self.niceness),
            BasicColumn::StartTime => format!("{}", self.start_time),
            BasicColumn::VSize => format!("{}", self.vsize),
            BasicColumn::RSS => format!("{}", self.rss),
            BasicColumn::Threads => format!("{}", self.threads),
            BasicColumn::CpuTime => format!("{}", self.cpu_time),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::PID => self.pid.cmp(&other.pid),
            BasicColumn::PPID => self.ppid.cmp(&other.ppid),
            BasicColumn::State => self.state.cmp(&other.state),
            BasicColumn::Priority => self.priority.cmp(&other.priority),
            BasicColumn::Niceness => self.niceness.cmp(&other.niceness),
            BasicColumn::StartTime => self.start_time.cmp(&other.start_time),
            BasicColumn::VSize => self.vsize.cmp(&other.vsize),
            BasicColumn::RSS => self.rss.cmp(&other.rss),
            BasicColumn::Threads => self.threads.cmp(&other.threads),
            BasicColumn::CpuTime => self.cpu_time.cmp(&other.cpu_time),
        }
    }
}

fn gethelpdeskstring()->String{
    let mut string = String::new();
    string.push_str("Q - Quit\t");
    string.push_str("R - Refresh\t");
    string.push_str("↑↓ - Navigate\t");
    string.push_str("K - Kill Process\t");
    string.push_str("C - Kill Branch\t");
    string.push_str("S - Sleep Process\t");
    string.push_str("T - Terminate Process\t");
    string.push_str("H - More Help\t");
    


    string 
}

fn terminate_process(pid: usize){

    if (pid == procfs::process::Process::myself().unwrap().stat().unwrap().pid as usize) {
        return;
    }
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::SIGTERM).unwrap();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
}


fn sleep_process(pid: usize){

    if (pid == procfs::process::Process::myself().unwrap().stat().unwrap().pid as usize) {
        return;
    }
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::SIGSTOP).unwrap();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
}

fn kill_process(pid: usize){
    if (pid == procfs::process::Process::myself().unwrap().stat().unwrap().pid as usize) {
        return;
    }
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::SIGKILL).unwrap();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
}

fn kill_branch(pid: usize) {

    if (pid == procfs::process::Process::myself().unwrap().stat().unwrap().pid as usize) {
        return;
    }

    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::SIGKILL).unwrap();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
    for proc in new_procs.iter_mut(){
        if proc.stat().unwrap().ppid == pid as i32{
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
            kill_process(proc.pid as usize);
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }

    }

}

fn getsystemstring()->String{
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.refresh_cpu();
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu();

    let mut line1 = (format!("OS: {}", sys.name().unwrap())).pad_to_width_with_alignment(50, Alignment::Left);
    line1 += &(format!("Uptime: {:02}:{:02}:{:02}", sys.uptime()/3600, sys.uptime()%3600/60, sys.uptime()%3600%60).pad_to_width_with_alignment(50, Alignment::Left));
    line1 += &(format!("Total CPU%: {:.2}%", sys.global_cpu_info().cpu_usage()).as_str().pad_to_width_with_alignment(50, Alignment::Left));
    
    let mut line2 = (format!("#Disks: {} disks", sys.disks().len()).as_str().pad_to_width_with_alignment(50, Alignment::Left));
    line2 += &(format!("Total Memory: {:.2} GB", (sys.total_memory() as f64 /1e9)).as_str().pad_to_width_with_alignment(50, Alignment::Left));
    line2 += &(format!("Memory%: {:.2}%", (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0)).as_str().pad_to_width_with_alignment(50, Alignment::Left);
    
    let mut line3 = (format!("#CPUs: {} cores", sys.cpus().len()).as_str().pad_to_width_with_alignment(50, Alignment::Left));
    line3 += &(format!("Total Swap: {:.2} GB", sys.total_swap() as f64 /1e9).as_str().pad_to_width_with_alignment(50, Alignment::Left));
    line3 += &(format!("Swap%: {:.2}%", (sys.used_swap() as f64/ sys.total_swap() as f64)*100.0).as_str().pad_to_width_with_alignment(50, Alignment::Left));

    let mut sysString = String::new();
    sysString.push_str(line1.as_str());
    sysString.push_str("\n");
    sysString.push_str(line2.as_str());
    sysString.push_str(line3.as_str());

    // let mut string = String::new();
    // string.push_str("OS: ");
    // string += &sys.name().unwrap();
    // string.push_str("\nUptime: ");
    // string.push_str(format!("{:02}:", sys.uptime()/3600).as_str());
    // string.push_str(format!("{:02}:", sys.uptime()%3600/60).as_str());
    // string.push_str(format!("{:02}\n", sys.uptime()%3600%60).as_str());
    // string.push_str("CPU Count: ");
    // string.push_str(format!("{} cores \n", sys.cpus().len()).as_str());
    // string.push_str("Total Memory: ");
    // string.push_str(format!("{:.2} GB \n", (sys.total_memory() as f64 /1e9)).as_str());
    // string.push_str("Swap: ");
    // string.push_str(format!("{:.2} GB\n", sys.total_swap() as f64/ 1e9).as_str());
    // string.push_str("Disk: ");
    // string.push_str(format!("{} disks\n", sys.disks().len()).as_str());
    
    // sys.refresh_cpu();
    // std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    // sys.refresh_cpu();

    // string.push_str("Total CPU%: ");
    // string.push_str(format!("{:.2}%\n", sys.global_cpu_info().cpu_usage()).as_str());
    // string.push_str("Memory%: ");
    // string.push_str(format!("{:.2}%\n", (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0).as_str());
    // string.push_str("Swap%: ");
    // string.push_str(format!("{:.2}%\n", (sys.used_swap() as f64/ sys.total_swap() as f64)*100.0).as_str());

    sysString 
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut all_procs: Vec<procfs::process::Process> = all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
    
    let mut siv = cursive::default();
    siv.set_autorefresh(true);
    
    siv.load_toml(include_str!("/home/sara/Desktop/lpm/The-Linux-Process-Manager/theme.toml")).unwrap();
    let mut systeminfo = TextView::new(getsystemstring());
    let mut helpdesk = TextView::new(gethelpdeskstring());
    let mut layout = LinearLayout::new(Orientation::Vertical);

    let mut table = TableView::<TableProcess, BasicColumn>::new()

        .column(BasicColumn::Name, "Name", |c| {
            c.ordering(Ordering::Greater)
                .width_percent(14)
                .align(HAlign::Right)
        })
        .column(BasicColumn::PID, "PID", |c| {
            c.align(HAlign::Right)
            .width_percent(4)
        })
        .column(BasicColumn::PPID, "PPID", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(5)
        })
        .column(BasicColumn::State, "State", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(6)
        })
        .column(BasicColumn::Priority, "Priority", |c| {
            c.ordering(Ordering::Greater)
                .width_percent(8)
                .align(HAlign::Right)
        })
        .column(BasicColumn::Niceness, "Nice", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::StartTime, "StartTime", |c| {
            c.ordering(Ordering::Greater)
                .width_percent(8)
                .align(HAlign::Right)
        })
        .column(BasicColumn::VSize, "VSize", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::RSS, "RSS", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(4)
        })
        .column(BasicColumn::Threads, "Threads", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::CpuTime, "CPU Time", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        });

    let mut items = Vec::new();
    for p in all_procs {
        items.push(TableProcess {
            name: format!("{}", p.stat().unwrap().comm),
            pid: p.stat().unwrap().pid,
            ppid: p.stat().unwrap().ppid,
            state: p.stat().unwrap().state,
            priority: p.stat().unwrap().priority,
            niceness: p.stat().unwrap().nice,
            start_time: p.stat().unwrap().starttime,
            vsize: p.stat().unwrap().vsize/1e6 as u64,
            rss: p.stat().unwrap().rss,
            threads: p.stat().unwrap().num_threads,
            cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
        });
    }

    table.set_items(items);

    let cb_sink = siv.cb_sink().clone();

    let duration = std::time::Duration::from_millis(5000);
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(duration);
            cb_sink
                .send(Box::new(move |s| {
            //         s.call_on_name("table", |table: &mut TableView<tableProcess, BasicColumn>|  {
            //             let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            //             let mut items = Vec::new();
            //             for p in new_procs {
            //         items.push(tableProcess {
            //             name: format!("{}", p.stat().unwrap().comm),
            //             pid: p.stat().unwrap().pid,
            //             ppid: p.stat().unwrap().ppid,
            //             state: p.stat().unwrap().state,
            //             priority: p.stat().unwrap().priority,
            //             niceness: p.stat().unwrap().nice,
            //             start_time: p.stat().unwrap().starttime,
            //             vsize: format!("{:.2}", ((p.stat().unwrap().vsize as f64)/1e6)),
            //             rss: p.stat().unwrap().rss,
            //             threads: p.stat().unwrap().num_threads,
            //             cpu_time: format!("{}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/procfs::ticks_per_second() as f32))
            //     });
            // }
            // table.set_items(items);
            //         }
            //     );
                s.call_on_name("sysinfo", |sysinfo: &mut TextView| {
                    let mut new_sysinfo = String::new();
                    new_sysinfo = getsystemstring();
                    sysinfo.set_content(new_sysinfo);
                });
                })
            )
                .unwrap();
        }
        cb_sink.send(Box::new(|s| s.quit())).unwrap();
    });

   // siv.run();


    // table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
    //     siv.add_layer(
    //         Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
    //             .title("Sorted by")
    //             .button("Close", |s| {
    //                 s.pop_layer();
    //             }),
    //     );
    // });

    layout.add_child(Dialog::around((systeminfo.with_name("sysinfo").min_height(3).max_height(3).min_width(150).max_width(150))).title("SYSTEM INFO"));
    layout.add_child(Dialog::around(table.with_name("table").min_height(30).max_height(40).min_width(150)).title("PROCESS TABLE"));
    layout.add_child(Dialog::around((helpdesk.min_height(1).max_height(1).min_width(120)).max_width(150)).title("HELP DESK")); 

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('r', |s| {
        s.call_on_name("table", |table: &mut TableView<TableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
        // updating system info already updates every second
        // s.call_on_name("sysinfo", |sysinfo: &mut TextView| {
        //     let mut new_sysinfo = String::new();
        //     new_sysinfo = getsystemstring();
        //     sysinfo.set_content(new_sysinfo);
        // });
    });
    siv.add_global_callback('h', |s|{
        let mut string:String = "\n".to_string();
        string.push_str("run \"lpm\" for view of all processes\n");
        string.push_str("run \"lpm -p <pid>\" for view of specific process\n");
        string.push_str("run \"lpm -pp <ppid>\" to filter processes by ppid\n");
        string.push_str("run \"lpm -s <state>\" to filter processes by state\n");
        string.push_str("run \"lpm -n <name>\" to filter processes by name\n");
        string.push_str("run \"lpm -h\" for more help\n\n");
        string.push_str("click on a column header to sort processes by that column field\n");
        s.add_layer(Dialog::text(string)
        .title("HELP")
        .button("Done", |s| {s.pop_layer();}));
    });
    siv.add_global_callback('k', |s|{
        s.call_on_name("table", |table: &mut TableView<TableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid, 
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            let currentpid = items[currentitem].pid;
            kill_process(currentpid as usize);
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
    });
    siv.add_global_callback('b', |s|{
        s.call_on_name("table", |table: &mut TableView<TableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid, 
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            let currentpid = items[currentitem].pid;
            kill_branch(currentpid as usize);
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
    });
    siv.add_global_callback('s', |s|{
        s.call_on_name("table", |table: &mut TableView<TableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid, 
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            let currentpid = items[currentitem].pid;
            sleep_process(currentpid as usize);
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
    });
    siv.add_global_callback('t', |s|{
        s.call_on_name("table", |table: &mut TableView<TableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid, 
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            let currentpid = items[currentitem].pid;
            terminate_process(currentpid as usize);
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(TableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize/1e6 as u64,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: (p.stat().unwrap().utime + p.stat().unwrap().stime) as i32/(procfs::ticks_per_second() as i32)
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
    });
    siv.add_global_callback('d', |s|{
        s.load_toml(include_str!("/home/sara/Desktop/lpm/The-Linux-Process-Manager/themedark.toml")).unwrap();
    });
    siv.add_global_callback('l', |s|{
        s.load_toml(include_str!("/home/sara/Desktop/lpm/The-Linux-Process-Manager/theme.toml")).unwrap();
    });
    siv.add_layer(layout);
    
    //siv.add_layer(Dialog::around(table.with_name("table").min_height(50).min_width(150)).title("Process Table"));
    siv.run();
}

