use mackerel_plugin::{graph, Graph, Plugin};
use std::collections::HashMap;

pub struct UptimePlugin {}

impl Plugin for UptimePlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        Ok(HashMap::from([(
            "uptime.uptime".to_owned(),
            uptime_lib::get()
                .map_err(|err| err.to_string())?
                .as_secs_f64(),
        )]))
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![graph! {
            name: "uptime",
            label: "Uptime",
            unit: "seconds",
            metrics: [
                { name: "uptime", label: "uptime" },
            ],
        }]
    }
}
