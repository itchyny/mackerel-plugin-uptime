extern crate mackerel_plugin;
extern crate mackerel_plugin_uptime;

use mackerel_plugin::Plugin;
use mackerel_plugin_uptime::UptimePlugin;

fn main() {
    let plugin = UptimePlugin {};
    match plugin.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("mackerel-plugin-uptime: {}", err);
            std::process::exit(1);
        }
    }
}
