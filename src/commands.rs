use crate::SessionConfig;
use std::{
    io,
    process::{Child, Command},
    thread,
    time::Duration,
};

const MAX_RETRIES: usize = 5;
const DELAYS_MS: Duration = Duration::from_millis(150);

pub enum ZellijCommand<'a> {
    NewTab {
        session_name: &'a str,
        tab_name: Option<&'a str>,
    },
    GoToTab {
        session_name: &'a str,
        tab_index: usize,
    },
    RenameTab {
        session_name: &'a str,
        new_name: &'a str,
    },
    WriteChars {
        session_name: &'a str,
        chars: &'a str,
    },
}

impl<'a> ZellijCommand<'a> {
    pub fn execute(&self) -> io::Result<Child> {
        match self {
            ZellijCommand::NewTab {
                session_name,
                tab_name,
            } => {
                log::info!("NewTab cmd with params: {{ session_name: {}, tab_name: {:?} }}", 
                    session_name, tab_name
                );
                let mut args =
                    vec!["--session", session_name, "action", "new-tab"];
                if let Some(name) = tab_name {
                    args.push("--name");
                    args.push(name);
                }
                let result = Command::new("zellij").args(&args).spawn();
                if let Err(e) = &result {
                    log::error!("Failed to create new tab: {}", e);
                }
                result
            }
            ZellijCommand::GoToTab {
                session_name,
                tab_index,
            } => {
                log::info!(
                    "GoToTab cmd with params {{ session_name: {}, tab_index: {} }}",
                    session_name,
                    tab_index
                );
                let idx = tab_index.to_string();
                let args =
                    vec!["--session", session_name, "action", "go-to-tab", &idx];
                let result = Command::new("zellij").args(&args).spawn();
                if let Err(e) = &result {
                    log::error!("Failed to go-to-tab: {}", e);
                }
                result
            }
            ZellijCommand::RenameTab {
                session_name,
                new_name,
            } => {
                log::info!(
                    "RenameTab cmd with params {{ session_name: {}, new_name: {} }}",
                    session_name,
                    new_name
                );
                let args = vec![
                    "--session",
                    session_name,
                    "action",
                    "rename-tab",
                    new_name,
                ];
                let result = Command::new("zellij").args(&args).spawn();
                if let Err(e) = &result {
                    log::error!("Failed to rename-tab: {}", e);
                }
                result
            }
            ZellijCommand::WriteChars {
                session_name,
                chars,
            } => {
                log::info!(
                    "WriteChars cmd with params {{ session_name: {}, chars: {} }}",
                    session_name,
                    chars
                );
                let command_with_enter = format!("{}\n", &chars);
                let args = vec![
                    "--session",
                    session_name,
                    "action",
                    "write-chars",
                    &command_with_enter,
                ];
                let result = Command::new("zellij").args(&args).spawn();
                if let Err(e) = &result {
                    log::error!("Failed to rename-tab: {}", e);
                }
                result
            }
        }
    }
}

pub fn start_session(config: &SessionConfig) {
    log::debug!("Running zellij session: {}", &config.session_name);

    // Check session exist, if yes fail with error!
    if check_session_exists(&config.session_name) {
        panic!("Session {} already exists!", &config.session_name);
    }
    // Create new session with name as in config
    // this will create child process to this cli tool which
    // should be released at the end of this function
    let mut session_child = Command::new("zellij")
        .args(["attach", "--create", &config.session_name])
        .spawn()
        .expect("Failed to start Zellij session");
    // Have to wait a bit till it will be created
    for _ in 1..MAX_RETRIES {
        if check_session_exists(&config.session_name) {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    // Open required amount of tabs, notice one should be open on session create
    for _ in 1..config.tabs.len() {
        let res = ZellijCommand::NewTab {
            session_name: config.session_name.as_str(),
            tab_name: None,
        }
        .execute();
        res.unwrap().wait().unwrap();
    }
    thread::sleep(DELAYS_MS);
    // Change tab names and run commands per tab (run common shell command too)
    let mut focus_idx = 1;
    for (idx, tab) in config.tabs.iter().enumerate() {
        // Store focused tab index
        if tab.focus {
            focus_idx = idx + 1;
        }
        // focus tab
        let res = ZellijCommand::GoToTab {
            session_name: config.session_name.as_str(),
            tab_index: idx + 1,
        }
        .execute();
        res.unwrap().wait().unwrap();
        thread::sleep(DELAYS_MS);
        // raname tab
        let res = ZellijCommand::RenameTab {
            session_name: config.session_name.as_str(),
            new_name: &tab.name,
        }
        .execute();
        res.unwrap().wait().unwrap();
        thread::sleep(DELAYS_MS);
        // call shell_command_before
        let res = ZellijCommand::WriteChars {
            session_name: config.session_name.as_str(),
            chars: format!("{}\n", config.shell_command_before).as_str(),
        }
        .execute();
        res.unwrap().wait().unwrap();
        thread::sleep(DELAYS_MS);
        // call required commands in tab
        for command in tab.commands.iter() {
            let res = ZellijCommand::WriteChars {
                session_name: config.session_name.as_str(),
                chars: format!("{}\n", command).as_str(),
            }
            .execute();
            res.unwrap().wait().unwrap();
            thread::sleep(DELAYS_MS);
        }
    }

    // Focus required tab
    let res = ZellijCommand::GoToTab {
        session_name: &config.session_name,
        tab_index: focus_idx,
    }
    .execute();
    res.unwrap().wait().unwrap();

    let _ = session_child.wait().expect("Failed to wait on Zellij");
}

fn check_session_exists(session_name: &str) -> bool {
    // get sessions list
    let output = Command::new("zellij")
        .args(["list-sessions"])
        .output()
        .expect("Failed to execute Zellij 'list-sessions' command");

    let sessions = String::from_utf8_lossy(&output.stdout);
    sessions.contains(session_name)
}
