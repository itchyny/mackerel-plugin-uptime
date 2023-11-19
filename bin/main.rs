use mackerel_plugin::Plugin;
use mackerel_plugin_uptime::UptimePlugin;

fn main() {
    if let Err(err) = (UptimePlugin {}).run() {
        eprintln!("mackerel-plugin-uptime: {}", err);
        std::process::exit(1);
    }
}
