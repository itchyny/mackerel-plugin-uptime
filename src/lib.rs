extern crate mackerel_plugin;
extern crate uptime_lib;

use mackerel_plugin::*;
use std::collections::HashMap;

pub struct UptimePlugin {}

impl Plugin for UptimePlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        let mut metrics = HashMap::new();
        let uptime = uptime_lib::get()?;
        metrics.insert("uptime.uptime".to_string(), uptime.as_secs_f64());
        Ok(metrics)
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![graph! {
            name: "uptime",
            label: "Uptime",
            unit: "integer",
            metrics: [
                { name: "uptime", label: "uptime (sec)" },
            ]
        }]
    }
}
