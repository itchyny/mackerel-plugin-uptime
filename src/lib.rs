extern crate libc;
#[macro_use]
extern crate mackerel_plugin;
extern crate time;

use std::collections::HashMap;
use std::mem;
use mackerel_plugin::*;

pub struct UptimePlugin {}

#[cfg(target_os = "linux")]
fn get_uptime() -> Result<f64, String> {
    let mut info: libc::sysinfo = unsafe { mem::zeroed() };
    unsafe {
        let ret = libc::sysinfo(&mut info);
        if ret == 0 {
            Ok(info.uptime as f64)
        } else {
            Err("sysinfo failed".to_string())
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
fn get_uptime() -> Result<f64, String> {
    let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boottime: libc::timeval = unsafe { mem::zeroed() };
    let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
    unsafe {
        let ret = libc::sysctl(
            &mut request[0],
            2,
            &mut boottime as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        );
        if ret == 0 {
            Ok((time::now().to_timespec().sec - boottime.tv_sec) as f64)
        } else {
            Err("sysctl failed".to_string())
        }
    }
}

impl Plugin for UptimePlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        let mut metrics = HashMap::new();
        let uptime = get_uptime()?;
        metrics.insert("uptime.uptime_sec".to_string(), uptime);
        Ok(metrics)
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![
            graph! {
                name: "uptime",
                label: "Uptime",
                unit: "integer",
                metrics: [
                    { name: "uptime_sec", label: "uptime (sec)" },
                ]
            },
        ]
    }
}
