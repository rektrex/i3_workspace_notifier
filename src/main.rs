use i3ipc::I3Connection;
use std::process::Command;

fn main() {
    let mut connection = I3Connection::connect().unwrap();

    let workspaces = connection.get_workspaces().unwrap().workspaces;

    let mut notification = String::new();

    let mut prev = 0;

    for workspace in workspaces.iter() {
        let diff = workspace.num - prev;
        for _ in 1..diff {
            notification += "<span foreground='#e76d6d'>●</span> "
        }

        if workspace.visible {
            notification += "<span foreground='#88c563'>●</span> ";
        } else {
            notification += "● ";
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
