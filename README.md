# DevCli

The goal of this project is to provide a cli to hasten my dev process and to create interface for the tools I use.

## Eclipse Che

Eclipse che is my main env that allow me to dev in my personal time (and i love it). This project is born from [this issue](https://github.com/eclipse/che/issues/22812) that follow my thinking process of loving project with source openly available.

There is also a redhat release of che that is called [DevSpace](https://developers.redhat.com/products/openshift-dev-spaces/overview).

### Commands

Inside of this module there will be multiple categories of commands that will be available, like:

- `dev_cli code` - Commands that are related to vscode manipulation (Open che workspace in vscode desktop)
- `dev_cli workspace` - Commands that are related to workspace manipulation (Info/Get/List/Start/Stop/...)
- `dev_cli shell` - Commands that are related to shell manipulation (ItShell/ExecShell/...)
- `dev_cli completion` - Commands that are related to completion manipulation (Bash/Zsh/...)

#### Code - Working

- `dev_cli code open` - Open the current workspace in vscode desktop
- `dev_cli code check` - Check if the needed vscode extension are installed and install them if needed

#### Workspace - Missing restart from local

- `dev_cli workspace get` - Output workspace info, and if run directly in a workspace, output the current workspace info, either ask the user to choose a workspace (output could be json or yaml)
- `dev_cli workspace get_container` - Get a workspace's container by name, by default the current if in one
- `dev_cli workspace list` - List all the workspaces
- `dev_cli workspace start` - Start a workspace by name
- `dev_cli workspace stop` - Stop a workspace by name
- `dev_cli workspace restart` - Restart a workspace by name
- `dev_cli workspace restart_from_local` - Restart from local devfile

#### Shell - TODO

- `dev_cli shell it` - Start an interactive shell in the current workspace
- `dev_cli shell exec` - Execute a command in the current workspace

#### Completion - Working

- `dev_cli completion -s $TERMINAL_KIND` - Output $TERMINAL_KIND completion script

## MonoRepo

Based on the [Rust NX](https://github.com/cammisuli/monodon/tree/main/packages/rust) template.

```bash
nx generate @monodon/rust:library my-rust-lib # Create lib
nx generate @monodon/rust:binary my-rust-app # Create app
```
