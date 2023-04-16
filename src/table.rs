pub mod table{

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub enum BasicColumn {
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
    pub struct tableProcess {
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
}

