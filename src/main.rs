use std::process::Command;
use std::time::Duration;

pub struct SessionConfig {
    session_name: String,
}

fn start_zellij_session(config: &SessionConfig) {
    // This is a placeholder function. You would implement the logic to start Zellij
    // and create panes according to the configuration here.

    // Use the Command struct from the std::process module to run Zellij.
    println!("Running zellij session: {}", &config.session_name);
    let mut child = Command::new("zellij")
        .args(&["attach", "--create", &config.session_name])
        .spawn()
        .expect("Failed to start Zellij session");

    std::thread::sleep(Duration::from_millis(1500));

    // zellij --session tst action new-tab --name code
    println!("Creating new zellij tab");
    let _ = Command::new("zellij")
        .args(&[
            "--session",
            &config.session_name,
            "action",
            "new-tab",
            "--name",
            "code",
        ])
        .spawn()
        .expect("Failed to create new pane");

    // Here you would add the logic to configure the panes as needed.
    // ...

    // It is important to handle the child process appropriately.
    // For a simple synchronous example, we can just wait for it to finish.
    let _result = child.wait().expect("Failed to wait on Zellij");
}

fn main() {
    let conf = SessionConfig {
        session_name: "test".into(),
    };
    start_zellij_session(&conf);
}
