extern crate libc;
#[macro_use]
extern crate mackerel_plugin;

use std::collections::HashMap;
use std::mem;
use mackerel_plugin::*;

pub struct UptimePlugin {}

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
