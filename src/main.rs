use i3ipc::I3Connection;
use std::process::Command;
use std::fs;
use std::collections::HashMap;
use dirs;

fn main() {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join("i3_workspace_notifier.conf");
    let contents = fs::read_to_string(config_path).unwrap();

    let mut colors = HashMap::new();

    // default colors
    colors.insert("visible", "#88c563");
    colors.insert("non-visible", "#fafafa");
    colors.insert("non-existent", "#e76d6d");

    for line in contents.lines() {
        if line.trim().is_empty() { continue; }

        let pair: Vec<&str> = line.split(":").collect();
        colors.insert(pair[0].trim(), pair[1].trim());
    }

    let visible_notification = format!("<span foreground='{}'>●</span> ", colors.get("visible").unwrap());
    let non_visible_notification = format!("<span foreground='{}'>●</span> ", colors.get("non-visible").unwrap());
    let non_existent_notification = format!("<span foreground='{}'>●</span> ", colors.get("non-existent").unwrap());

    let mut connection = I3Connection::connect().unwrap();

    let workspaces = connection.get_workspaces().unwrap().workspaces;

    let mut notification = String::new();

    let mut prev = 0;

    for workspace in workspaces.iter() {
        let diff = workspace.num - prev;
        for _ in 1..diff {
            notification += &non_existent_notification;
        }

        if workspace.visible {
            notification += &visible_notification;
        } else {
            notification += &non_visible_notification;
        }

        prev = workspace.num;
    }


    Command::new("notify-send")
            .arg("-u")
            .arg("Low")
            .arg(" ")
            .arg(notification.trim_end())
            .output()
            .expect("command failed");
}
