use std::io::Cursor;

use mackerel_plugin::Plugin;
use mackerel_plugin_uptime::UptimePlugin;

#[test]
fn plugin_output_values() {
    let plugin = UptimePlugin {};
    let mut out = Cursor::new(Vec::new());
    assert!(plugin.output_values(&mut out).is_ok());
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert!(out_str.contains("uptime.uptime\t"));
}

#[test]
fn plugin_output_definitions() {
    let plugin = UptimePlugin {};
    let mut out = Cursor::new(Vec::new());
    assert!(plugin.output_definitions(&mut out).is_ok());
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert!(out_str.starts_with("# mackerel-agent-plugin\n"));
}
