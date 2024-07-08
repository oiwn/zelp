# Zelp

Something like Tmuxp but for Zellij 

### Config Example

```
SessionConfig(
    session_name: "zelp-test-session",
    shell_command_before: "export RUST_LOG=info",
    tabs: [
        ( name: "code", focus: true, commands: ["helix"] ),
        ( name: "cmd1", commands: ["clear"] ),
        ( name: "monitoring", commands: ["btm"] ),
        ( name: "cmd2"),
    ],
)
```

### Features

Very basic for now

- [x] Load tabs and run commands inside them
- [x] Run common commands inside tabs (like `conda` or `export RUST_LOG=info`)
- [ ] Can split tabs to panes (look how tmuxp implemented it)
- [ ] Figure out maybe pipe commands directly into the zellij server
