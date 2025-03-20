use std::fmt::Display;

// Struct for basic OS information
pub struct OsInfo {
    pub hostname: String,
    pub uptime: u64,
    pub os_type: String,
    pub os_version: String,
    pub os_arch: String,
}

impl Display for OsInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hostname: {}, Uptime: {}, OS Type: {}, OS Version: {}, OS Arch: {}",
            self.hostname, self.uptime, self.os_type, self.os_version, self.os_arch
        )
    }
}
