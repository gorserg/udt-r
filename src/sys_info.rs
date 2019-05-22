use mac_address::get_mac_address;
use std::fmt;
use sys_info::{disk_info, hostname, loadavg, os_release, os_type};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub os_version: String,
    pub os_name: String,
    pub hostname: String,
    pub mac_address: String,
    pub load_avg: String,
    pub disk_info: String,
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        let placeholder = "<unknown>";

        let load_avg = loadavg()
            .map(|val| format!("{:0.2} {:0.2} {:0.2}", val.one, val.five, val.fifteen))
            .unwrap_or(placeholder.to_string());
        let disk_info = disk_info()
            .map(|val| format!("{}/{}", val.free, val.total))
            .unwrap_or(placeholder.to_string());

        let si = SystemInfo {
            os_version: os_release().unwrap_or_default(),
            os_name: os_type().unwrap_or_default(),
            hostname: hostname().unwrap_or_default(),
            mac_address: mac_address(),
            load_avg: load_avg,
            disk_info: disk_info,
        };
        si
    }
}
fn mac_address() -> String {
    match get_mac_address() {
        Ok(Some(address)) => address.to_string(),
        _ => "".to_string(),
    }
}
