
// // Crate Dependencies ---------------------------------------------------------
// // ----------------------------------------------------------------------------
// extern crate cursive;
// extern crate cursive_table_view;
// extern crate rand;

// // STD Dependencies -----------------------------------------------------------
// // ----------------------------------------------------------------------------
// use std::cmp::Ordering;

// // External Dependencies ------------------------------------------------------
// // ----------------------------------------------------------------------------
// use cursive::align::HAlign;
// use cursive::traits::*;
// use cursive::views::*;
// use sysinfo::{System, ProcessExt, SystemExt, CpuExt};
// use procfs::process::*;
// use cursive::direction::Orientation;
// //use tokio::time::timeout;
// use std::thread;
// use std::time::SystemTime;
// use pad::{PadStr, Alignment};
// //include table.rs
// //use crate::table::{tableProcess, BasicColumn};

// // Modules --------------------------------------------------------------------
// // ----------------------------------------------------------------------------
// use cursive_table_view::{TableView, TableViewItem};

// #[derive(Copy, Clone, PartialEq, Eq, Hash)]
// enum BasicColumn {
//     Name, // Process Name
//     PID, // Process ID
//     PPID, // Parent Process ID
//     State, // Process State
//     Priority, // Process Priority
//     Niceness, // Process Niceness
//     StartTime, // Process Start Time
//     VSize, // Virtual Memory Size
//     RSS, // Resident Set Size
//     Threads, // Number of Threads
//     CPU_Time, // CPU Usage
// }

// impl BasicColumn {
//     fn as_str(&self) -> &str {
//         match *self {
//             BasicColumn::Name => "Name",
//             BasicColumn::PID => "PID",
//             BasicColumn::PPID => "PPID",
//             BasicColumn::State => "State",
//             BasicColumn::Priority => "Priority",
//             BasicColumn::Niceness => "Niceness",
//             BasicColumn::StartTime => "Start Time",
//             BasicColumn::VSize => "VSize",
//             BasicColumn::RSS => "RSS",
//             BasicColumn::Threads => "Threads",
//             BasicColumn::CPU_Time => "CPU Time",
//         }
//     }
// }

// #[derive(Clone, Debug)]
// struct tableProcess {
//     name: String,
//     pid: i32,
//     ppid: i32,
//     state: char,
//     priority: i64,
//     niceness: i64,
//     start_time: u64,
//     vsize: String,
//     rss: u64,
//     threads: i64,
//     cpu_time: String,
// }

// impl TableViewItem<BasicColumn> for tableProcess {
//     fn to_column(&self, column: BasicColumn) -> String {
//         match column {
//             BasicColumn::Name => self.name.to_string(),
//             BasicColumn::PID => format!("{}", self.pid),
//             BasicColumn::PPID => format!("{}", self.ppid),
//             BasicColumn::State => format!("{}", self.state),
//             BasicColumn::Priority => format!("{}", self.priority),
//             BasicColumn::Niceness => format!("{}", self.niceness),
//             BasicColumn::StartTime => format!("{}", self.start_time),
//             BasicColumn::VSize => format!("{}", self.vsize),
//             BasicColumn::RSS => format!("{}", self.rss),
//             BasicColumn::Threads => format!("{}", self.threads),
//             BasicColumn::CPU_Time => format!("{}", self.cpu_time),
//         }
//     }

//     fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
//     where
//         Self: Sized,
//     {
//         match column {
//             BasicColumn::Name => self.name.cmp(&other.name),
//             BasicColumn::PID => self.pid.cmp(&other.pid),
//             BasicColumn::PPID => self.ppid.cmp(&other.ppid),
//             BasicColumn::State => self.state.cmp(&other.state),
//             BasicColumn::Priority => self.priority.cmp(&other.priority),
//             BasicColumn::Niceness => self.niceness.cmp(&other.niceness),
//             BasicColumn::StartTime => self.start_time.cmp(&other.start_time),
//             BasicColumn::VSize => self.vsize.cmp(&other.vsize),
//             BasicColumn::RSS => self.rss.cmp(&other.rss),
//             BasicColumn::Threads => self.threads.cmp(&other.threads),
//             BasicColumn::CPU_Time => self.cpu_time.cmp(&other.cpu_time),
//         }
//     }
// }

// fn gethelpdeskstring()->String{
//     let mut string = String::new();
//     string.push_str("Q - Quit\t");
//     string.push_str("R - Refresh\t");
//     string.push_str("↑↓ - Navigate Through Process Table\t");
//     string.push_str("K - Kill Selected Process\t");
//     string.push_str("H - More Help");

//     string 
// }

// fn kill_process(pid: usize) {
//     let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
//     if pid == procfs::process::Process::myself().unwrap().stat().unwrap().pid as usize {
//         println!("Cannot kill self");
//         return;
//     }
//     let mut sys = System::new_all();
//     sys.refresh_all();
//     let mut process = sys.process(sysinfo::Pid::from(pid)).unwrap();
//     std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
//     process.kill();
//     std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    
// }

// fn getsystemstring()->String{
//     let mut sys = System::new_all();
//     sys.refresh_all();

//     sys.refresh_cpu();
//     std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
//     sys.refresh_cpu();

//     let mut line1 = (format!("OS: {}", sys.name().unwrap())).pad_to_width_with_alignment(40, Alignment::Left);
//     line1 += &(format!("Uptime: {:02}:{:02}:{:02}", sys.uptime()/3600, sys.uptime()%3600/60, sys.uptime()%3600%60).pad_to_width_with_alignment(40, Alignment::Left));
//     line1 += &(format!("Total CPU%: {:.2}%", sys.global_cpu_info().cpu_usage()).as_str().pad_to_width_with_alignment(40, Alignment::Left));
    
//     let mut line2 = format!("#Disks: {} disks", sys.disks().len()).as_str().pad_to_width_with_alignment(40, Alignment::Left);
//     line2 += &(format!("Total Memory: {:.2} GB", (sys.total_memory() as f64 /1e9)).as_str().pad_to_width_with_alignment(40, Alignment::Left));
//     line2 += &(format!("Memory%: {:.2}%", (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0)).as_str().pad_to_width_with_alignment(40, Alignment::Left);
    
//     let mut line3 = format!("#CPUs: {} cores", sys.cpus().len()).as_str().pad_to_width_with_alignment(40, Alignment::Left);
//     line3 += &(format!("Total Swap: {:.2} GB", sys.total_swap() as f64 /1e9).as_str().pad_to_width_with_alignment(40, Alignment::Left));
//     line3 += &(format!("Swap%: {:.2}%", (sys.used_swap() as f64/ sys.total_swap() as f64)*100.0).as_str().pad_to_width_with_alignment(40, Alignment::Left));

//     let mut sysString = String::new();
//     sysString.push_str(line1.as_str());
//     sysString.push_str("\n");
//     sysString.push_str(line2.as_str());
//     sysString.push_str("\n");
//     sysString.push_str(line3.as_str());


//     sysString 
// }


// fn main() {
//     let mut rng = rand::thread_rng();
//     let mut all_procs: Vec<procfs::process::Process> = all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
    
//     let mut siv = cursive::default();
//     siv.set_fps(30);
//     siv.set_autorefresh(true);
    
//     siv.load_toml(include_str!("/home/mohamedshaalan/Desktop/lpm/lpm/theme.toml")).unwrap();
//     let mut systeminfo = TextView::new(getsystemstring());
//     let mut helpdesk = TextView::new(gethelpdeskstring());
//     let mut layout = LinearLayout::new(Orientation::Vertical);

//     let mut table = TableView::<tableProcess, BasicColumn>::new()


//         .column(BasicColumn::Name, "Name", |c| {
//             c.ordering(Ordering::Greater)
//                 .width_percent(14)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::PID, "PID", |c| {
//             c.align(HAlign::Left)
//             .width_percent(5)
//         })
//         .column(BasicColumn::PPID, "PPID", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//                 .width_percent(6)
//         })
//         .column(BasicColumn::State, "State", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//                 .width_percent(6)
//         })
//         .column(BasicColumn::Priority, "Priority", |c| {
//             c.ordering(Ordering::Greater)
//                 .width_percent(8)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::Niceness, "Nice", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::StartTime, "StartTime", |c| {
//             c.ordering(Ordering::Greater)
//                 .width_percent(8)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::VSize, "VSize", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::RSS, "RSS", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//                 .width_percent(4)
//         })
//         .column(BasicColumn::Threads, "Threads", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//         })
//         .column(BasicColumn::CPU_Time, "CPU Time", |c| {
//             c.ordering(Ordering::Greater)
//                 .align(HAlign::Left)
//         });

//     let mut items = Vec::new();
//     let mut total_cpu_time =0;
//     for p in &all_procs{
//         total_cpu_time = total_cpu_time + p.stat().unwrap().utime + p.stat().unwrap().stime;
//     }
//     for p in all_procs {
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
//            // cpu_time: format!("{}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/(procfs::ticks_per_second() as f32)))
//            cpu_time: format!("{:.3}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/(total_cpu_time as f32)*10 as f32)),
//         });
//     }

//     table.set_items(items);

//     // table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
//     //     siv.add_layer(
//     //         Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
//     //             .title("Sorted by")
//     //             .button("Close", |s| {
//     //                 s.pop_layer();
//     //             }),
//     //     );
//     // });

//     layout.add_child(Dialog::around(systeminfo.with_name("sysinfo").min_height(3).max_height(3).min_width(120).max_width(120)).title("SYSTEM INFO"));
//     layout.add_child(Dialog::around(table.with_name("table").min_height(20).max_height(30).min_width(120)).title("PROCESS TABLE"));
//     layout.add_child(Dialog::around((helpdesk.min_height(1).max_height(1).min_width(120)).max_width(120)).title("HELP DESK")); 

//     siv.add_global_callback('q', |s| s.quit());
//     siv.add_global_callback('r', |s| {
//         s.call_on_name("table", |table: &mut TableView<tableProcess, BasicColumn>| {
//             let mut currentitem:usize = table.item().unwrap_or(1);
//             table.clear();
//             let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
//             let mut items = Vec::new();
//             let mut total_cpu_time = 0;
//             for p in &new_procs{
//                 total_cpu_time = total_cpu_time + p.stat().unwrap().utime + p.stat().unwrap().stime;
//             }
//             for p in new_procs {
//                 items.push(tableProcess {
//                     name: format!("{}", p.stat().unwrap().comm),
//                     pid: p.stat().unwrap().pid,
//                     ppid: p.stat().unwrap().ppid,
//                     state: p.stat().unwrap().state,
//                     priority: p.stat().unwrap().priority,
//                     niceness: p.stat().unwrap().nice,
//                     start_time: p.stat().unwrap().starttime,
//                     vsize: format!("{:.2}", ((p.stat().unwrap().vsize as f64)/1e6)),
//                     rss: p.stat().unwrap().rss,
//                     threads: p.stat().unwrap().num_threads,
//                     cpu_time: format!("{:.3}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/(total_cpu_time as f32)*10 as f32)),
//                 });
//             }
//             table.set_items(items);
//             table.set_selected_item(currentitem);
//         });
//         s.call_on_name("sysinfo", |sysinfo: &mut TextView| {
//             let mut new_sysinfo = String::new();
//             new_sysinfo = getsystemstring();
//             sysinfo.set_content(new_sysinfo);
//         });
//     });
//     siv.add_global_callback('h', |s|{
//         let mut string:String = "\n".to_string();
//         string.push_str("run \"lpm\" for view of all processes\n");
//         string.push_str("run \"lpm -p <pid>\" for view of specific process\n");
//         string.push_str("run \"lpm -pp <ppid>\" to filter processes by ppid\n");
//         string.push_str("run \"lpm -s <state>\" to filter processes by state\n");
//         string.push_str("run \"lpm -n <name>\" to filter processes by name\n");
//         string.push_str("run \"lpm -h\" for more help\n\n");
//         string.push_str("click on a column header to sort processes by that column field\n");
//         s.add_layer(Dialog::text(string)
//         .title("HELP")
//         .button("Done", |s| {s.pop_layer();}));
//     });
//     siv.add_global_callback('k', |s|{
//         s.call_on_name("table", |table: &mut TableView<tableProcess, BasicColumn>| {
//             let mut currentitem:usize = table.item().unwrap_or(1);
//             let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
//             let mut items = Vec::new();
//             let mut total_cpu_time =0;
//             for p in &new_procs{
//                 total_cpu_time = total_cpu_time + p.stat().unwrap().utime + p.stat().unwrap().stime;
//             }
//             for p in new_procs {
//                 items.push(tableProcess {
//                     name: format!("{}", p.stat().unwrap().comm),
//                     pid: p.stat().unwrap().pid,
//                     ppid: p.stat().unwrap().ppid,
//                     state: p.stat().unwrap().state,
//                     priority: p.stat().unwrap().priority,
//                     niceness: p.stat().unwrap().nice,
//                     start_time: p.stat().unwrap().starttime,
//                     vsize: format!("{:.2}", ((p.stat().unwrap().vsize as f64)/1e6)),
//                     rss: p.stat().unwrap().rss,
//                     threads: p.stat().unwrap().num_threads,
//                     //cpu_time: format!("{}", (p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/procfs::ticks_per_second() as f32),
//                     cpu_time: format!("{:.3}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/(total_cpu_time as f32)*10 as f32)),
//                 });
//             }
//             let currentpid = items[currentitem].pid;
//             kill_process(currentpid as usize);
//             let mut currentitem:usize = table.item().unwrap_or(1);
//             table.clear();
//             let mut new_procs:Vec<procfs::process::Process> = procfs::process::all_processes().unwrap().into_iter().map(|x| x.unwrap()).collect();
//             let mut items = Vec::new();
//             let mut total_cpu_time =0;
//             for p in &new_procs{
//                 total_cpu_time = total_cpu_time + p.stat().unwrap().utime + p.stat().unwrap().stime;
//             }
//             for p in new_procs {
//                 items.push(tableProcess {
//                     name: format!("{}", p.stat().unwrap().comm),
//                     pid: p.stat().unwrap().pid,
//                     ppid: p.stat().unwrap().ppid,
//                     state: p.stat().unwrap().state,
//                     priority: p.stat().unwrap().priority,
//                     niceness: p.stat().unwrap().nice,
//                     start_time: p.stat().unwrap().starttime,
//                     vsize: format!("{:.2}", ((p.stat().unwrap().vsize as f64)/1e6)),
//                     rss: p.stat().unwrap().rss,
//                     threads: p.stat().unwrap().num_threads,
//                     //cpu_time: format!("{}", (p.stat().unwrap().utime as f32 + p.stat().unwrap().stime as f32)),
//                     cpu_time: format!("{:.3}", ((p.stat().unwrap().utime + p.stat().unwrap().stime) as f32/(total_cpu_time as f32)*10 as f32)),
//                     //procfs::ticks_per_second() as f32),

//                 });
//             }
//             table.set_items(items);
//             table.set_selected_item(currentitem);
//         });
//     });
//     siv.add_global_callback('d', |s|{
//         s.load_toml(include_str!("/home/mohamedshaalan/Desktop/lpm/lpm/themedark.toml")).unwrap();
//     });
//     siv.add_global_callback('l', |s|{
//         s.load_toml(include_str!("/home/mohamedshaalan/Desktop/lpm/lpm/theme.toml")).unwrap();
//     });
//     siv.add_layer(layout);
    
//     //siv.add_layer(Dialog::around(table.with_name("table").min_height(50).min_width(150)).title("Process Table"));
//     siv.run();
// }
// Crate Dependencies ---------------------------------------------------------
extern crate cursive;
extern crate cursive_tree_view;
extern crate rand;


// External Dependencies ------------------------------------------------------
use cursive::Cursive;
use cursive::CursiveExt;
use cursive::traits::*;
use cursive::views::*;


// Modules --------------------------------------------------------------------
use cursive_tree_view::{TreeView, Placement};
use cursive_tree_view::*;
use procfs::process::*;
use std::collections::HashMap;

//

fn main(){
    let mut tree = TreeView::new();
    
    tree.insert_item("root".to_string(), Placement::LastChild, 0);
    
    tree.insert_item("1".to_string(), Placement::LastChild, 0);
    tree.insert_item("2".to_string(), Placement::LastChild, 1);
    tree.insert_item("3".to_string(), Placement::LastChild, 2);

    //display this tree
    let mut siv = Cursive::default();
    siv.add_layer(
        Dialog::around(tree.with_name("tree").min_width(50).min_height(10))
            .title("Tree View")
            .button("Quit", |s| s.quit()),
    );
    siv.run();

}