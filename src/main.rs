
// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate cursive;
extern crate cursive_table_view;
extern crate rand;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;

use cursive::CursiveRunnable;
// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::*;
use cursive::theme;
use sysinfo::{System, ProcessExt, SystemExt, CpuExt};
use cursive::Cursive;
use procfs::process::{all_processes};
use cursive::direction::Orientation;
use core::time::Duration;
//use tokio::time::timeout;
use std::thread;
use std::time::SystemTime;
//include table.rs
//use crate::table::{tableProcess, BasicColumn};

// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
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
    CPU_Time, // CPU Usage
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
            BasicColumn::CPU_Time => "CPU Time",
        }
    }
}

#[derive(Clone, Debug)]
struct tableProcess {
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
    cpu_time: u64,
}

impl TableViewItem<BasicColumn> for tableProcess {
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
            BasicColumn::CPU_Time => format!("{}", self.cpu_time),
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
            BasicColumn::CPU_Time => self.cpu_time.cmp(&other.cpu_time),
        }
    }
}

fn gethelpdeskstring()->String{
    let mut string = String::new();
    string.push_str("Q - Quit\t");
    string.push_str("R - Refresh\t");
    string.push_str("↑↓ - Navigate Through Process Table\t");
    string.push_str("H - More Help");

    string 
}

fn kill_process(pid: usize) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut process = sys.process(sysinfo::Pid::from(pid)).unwrap();
    process.kill();
}

fn getsystemstring()->String{
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut string = String::new();

    string.push_str("System Information: \n");
    string.push_str("OS: ");
    string += &sys.name().unwrap();
    string.push_str("\nUptime: ");
    string.push_str(sys.uptime().to_string().as_str());
    string.push_str(" seconds\n");
    string.push_str("CPU: ");
    string.push_str(sys.cpus().len().to_string().as_str());
    string.push_str(" cores\n");
    string.push_str("Memory: ");
    string.push_str((sys.total_memory() as f64 /1e9).to_string().as_str());
    string.push_str(" GB\n");
    string.push_str("Swap: ");
    string.push_str(sys.total_swap().to_string().as_str());
    string.push_str(" bytes\n");
    string.push_str("Disk: ");
    string.push_str(sys.disks().len().to_string().as_str());
    string.push_str(" disks\n");
    SystemExt::refresh_cpu(&mut sys); // Refreshing CPU information.
    SystemExt::refresh_cpu(&mut sys); // Refreshing CPU information.

    let mut cpucount = 0;
    for cpu in sys.cpus() {
        string.push_str("CPU ");
        string.push_str(cpucount.to_string().as_str());
        string.push_str(" %: ");
        string.push_str(cpu.cpu_usage().to_string().as_str());
        string.push_str("\n");
        cpucount += 1;
    }
    string.push_str(sys.global_cpu_info().cpu_usage().to_string().as_str());
    // string.push_str(sys.cpu_load().unwrap().to_string().as_str());
    // string.push_str("")
    string.push_str("Memory%: ");
    //string += &((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0).to_string();
    string.push_str(((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0).to_string().as_str());
    //string.push_str(sys.memory().unwrap().to_string().as_str());
    // string.push_str("");
    // string.push_str("Swap%: ");
    // string.push_str(sys.swap().unwrap().to_string().as_str());
    // string.push_str("");
    // string.push_str(sys.networks().len().to_string().as_str());
    // string.push_str(" interfaces");

    string 
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut all_procs: Vec<procfs::process::Process> = all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
    
    let mut siv = cursive::default();
    siv.set_fps(1);
    siv.set_autorefresh(true);
    siv.load_toml(include_str!("/home/sara/Desktop/testing_cursive/theme.toml")).unwrap();
    let mut systeminfo = TextView::new(getsystemstring());
    let mut helpdesk = TextView::new(gethelpdeskstring());
    let mut layout = LinearLayout::new(Orientation::Vertical);

       
    let mut table = TableView::<tableProcess, BasicColumn>::new()


        .column(BasicColumn::Name, "Name", |c| {
            c.ordering(Ordering::Greater)
                .width_percent(14)
                .align(HAlign::Right)
        })
        .column(BasicColumn::PID, "PID", |c| {
            c.align(HAlign::Right)
        })
        .column(BasicColumn::PPID, "PPID", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::State, "State", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::Priority, "Priority", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::Niceness, "Nice", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::StartTime, "StartTime", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::VSize, "VSize", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::RSS, "RSS", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::Threads, "Threads", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        })
        .column(BasicColumn::CPU_Time, "CPU Time", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
        });

    let mut items = Vec::new();
    for p in all_procs {
        items.push(tableProcess {
            name: format!("{}", p.stat().unwrap().comm),
            pid: p.stat().unwrap().pid,
            ppid: p.stat().unwrap().ppid,
            state: p.stat().unwrap().state,
            priority: p.stat().unwrap().priority,
            niceness: p.stat().unwrap().nice,
            start_time: p.stat().unwrap().starttime,
            vsize: p.stat().unwrap().vsize,
            rss: p.stat().unwrap().rss,
            threads: p.stat().unwrap().num_threads,
            cpu_time: p.stat().unwrap().utime + p.stat().unwrap().stime,
        });
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    layout.add_child(Dialog::around((systeminfo.with_name("sysinfo").max_height(50).min_width(120))).title("SYSTEM INFO"));
    layout.add_child(Dialog::around(table.with_name("table").min_height(40).max_height(40).min_width(150)).title("PROCESS TABLE"));
    layout.add_child(Dialog::around((helpdesk.min_height(1).max_height(1).min_width(120))).title("HELP DESK")); 

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('r', |s| {
        s.call_on_name("table", |table: &mut TableView<tableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(tableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: p.stat().unwrap().utime + p.stat().unwrap().stime,
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
        s.call_on_name("sysinfo", |sysinfo: &mut TextView| {
            let mut new_sysinfo = String::new();
            new_sysinfo = getsystemstring();
            sysinfo.set_content(new_sysinfo);
        });
    });
    siv.add_global_callback('h', |s|{
        let mut string:String = "\n".to_string();
        string.push_str("run \"lpm\" for view of all processes\n");
        string.push_str("run \"lpm -p <pid>\" for view of specific process\n");
        string.push_str("run \"lpm -pp <ppid>\" to filter processes by ppid\n");
        string.push_str("run \"lpm -s <state>\" to filter processes by state\n");
        string.push_str("run \"lpm -n <name>\" to filter processes by name\n");
        string.push_str("run \"lpm -h\" for more help\n");
        s.add_layer(Dialog::text(string)
        .title("HELP")
        .button("Done", |s| {s.pop_layer();}));
    });
    siv.add_global_callback('k', |s|{
        s.call_on_name("table", |table: &mut TableView<tableProcess, BasicColumn>| {
            let mut currentitem:usize = table.item().unwrap_or(1);
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(tableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: p.stat().unwrap().utime + p.stat().unwrap().stime,
                });
            }
            let currentpid = items[currentitem].pid;
            kill_process(currentpid as usize);
            let mut currentitem:usize = table.item().unwrap_or(1);
            table.clear();
            let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
            let mut items = Vec::new();
            for p in new_procs {
                items.push(tableProcess {
                    name: format!("{}", p.stat().unwrap().comm),
                    pid: p.stat().unwrap().pid,
                    ppid: p.stat().unwrap().ppid,
                    state: p.stat().unwrap().state,
                    priority: p.stat().unwrap().priority,
                    niceness: p.stat().unwrap().nice,
                    start_time: p.stat().unwrap().starttime,
                    vsize: p.stat().unwrap().vsize,
                    rss: p.stat().unwrap().rss,
                    threads: p.stat().unwrap().num_threads,
                    cpu_time: p.stat().unwrap().utime + p.stat().unwrap().stime,
                });
            }
            table.set_items(items);
            table.set_selected_item(currentitem);
        });
    });
    siv.add_global_callback('d', |s|{
        s.load_toml(include_str!("/home/sara/Desktop/testing_cursive/themedark.toml")).unwrap();
    });
    siv.add_global_callback('l', |s|{
        s.load_toml(include_str!("/home/sara/Desktop/testing_cursive/theme.toml")).unwrap();
    });
    siv.add_layer(layout);
    
    //siv.add_layer(Dialog::around(table.with_name("table").min_height(50).min_width(150)).title("Process Table"));
    siv.run();
}
