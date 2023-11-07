use crate::SessionConfig;
use std::{process::Command, thread, time::Duration};

// NOTE: i think i'll need to implement it like this in future
// or check if it's possible to implement it as plugin
// enum ZellijCommand {
//     // Tabs
//     RenameTab {},
//     GoToTab {},
//     GoToTabName {},
//     NewTab {},
//     QueryTabNames {},
//     WriteChars {},
// }

pub fn start_session(config: &SessionConfig) {
    // This is a placeholder function. You would implement the logic to start Zellij
    // and create panes according to the configuration here.

    // Use the Command struct from the std::process module to run Zellij.
    println!("Running zellij session: {}", &config.session_name);
    let mut child = Command::new("zellij")
        .args(&["attach", "--create", &config.session_name])
        .spawn()
        .expect("Failed to start Zellij session");

    for _ in 0..10 {
        if check_session_exists(&config.session_name) {
            break;
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    }
    // Here need add the logic to configure the panes as needed.
    load_tabs(config);

    // It is important to handle the child process appropriately.
    // For a simple synchronous code, can just wait for it to finish.
    let _result = child.wait().expect("Failed to wait on Zellij");
}

fn check_session_exists(session_name: &str) -> bool {
    // get sessions list
    let output = Command::new("zellij")
        .args(&["list-sessions"])
        .output()
        .expect("Failed to execute Zellij 'list-sessions' command");

    let sessions = String::from_utf8_lossy(&output.stdout);
    sessions.contains(session_name)
}

fn load_tabs(config: &SessionConfig) {
    // std::thread::sleep(Duration::from_millis(2000));
    if let Some((first_tab, rest_of_tabs)) = config.tabs.split_first() {
        // zellij action rename-tab "alice the cat"
        println!("Renaming current zellij tab");
        let mut res = Command::new("zellij")
            .args(&[
                "--session",
                &config.session_name,
                "action",
                "rename-tab",
                &first_tab.name,
            ])
            .spawn()
            .expect("Failed to rename tab!");
        let _ = res.wait().expect("Renaming tab failed");

        // std::thread::sleep(Duration::from_millis(2000));

        for tab in rest_of_tabs {
            // zellij --session tst action new-tab --name code
            println!("Creating new zellij tab");
            let mut res = Command::new("zellij")
                .args(&[
                    "--session",
                    &config.session_name,
                    "action",
                    "new-tab",
                    "--name",
                    &tab.name,
                ])
                .spawn()
                .expect("Failed to create new pane");
            let _ = res.wait().expect("Waiting for tabs to load failed");
        }
    } else {
        println!("No tabs found");
    };

    // So here we are with all required tabs loaded and named properly
    // Now need to:
    // 1) focus on appropriate tab
    // 2) run commands inside tabs

    for tab in &config.tabs {
        // zellij action go-to-tab-name "blabla"
        println!("Move focus to tab: {}", &tab.name);
        let mut res = Command::new("zellij")
            .args(&[
                "--session",
                &config.session_name,
                "action",
                "go-to-tab-name",
                &tab.name,
            ])
            .spawn()
            .expect("Failed to move focus");
        let _ = res.wait().expect("Failed to run focus ");

        // zellij action write-chars "helix"
        for command in &tab.commands {
            let command_with_enter = format!("{}\n", &command);
            Command::new("zellij")
                .args(&["action", "write-chars", &command_with_enter])
                .status()
                .expect("Failed to write chars to Zellij");
        }
    }

    // focus on first for now
    Command::new("zellij")
        .args(&["action", "go-to-tab", "1"])
        .status()
        .expect("Failed to focus to tab");

    // // std::thread::sleep(Duration::from_millis(2000));
}
