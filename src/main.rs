// Clap is a crate used to parse CLI inputs
use std::{env, process::Command, process::Stdio};
use clap::{Args, Parser, Subcommand};
use anyhow::Result;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
enum LoggingLevel {
    #[default] 
    Default,
    Verbose
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct GlobalConfiguration {
    logging_level: LoggingLevel,
    git_command: String,
    edit_command: String,
    shell_command: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct RepositoryConfiguration {
    author: String,
    name: String,
    path: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Configuration {
    globals: GlobalConfiguration,
    repositories: Vec<RepositoryConfiguration>
}

// The derive attribute allows new items to be automatically generated for data structures.
#[derive(Debug, Parser)]
#[command(author, version)]
pub struct InputArguements {
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
    #[command(subcommand)]
    command: Option<ValidSubcommands>,
}

#[derive(Debug, Subcommand)]
pub enum ValidSubcommands {
    #[command(name = "notes")]
    NotesSubcommand(NotesSubcommand),
}

#[derive(Debug, Args)]
pub struct NotesSubcommand {
    #[arg(short = 'e', long = "edit")]
    edit: bool,
    #[arg(short = 'g', long = "git")]
    git: bool,
    #[arg(short = 'x', long = "terminal")]
    terminal: bool,
}

fn main() -> Result<()> {
    let cfg: Configuration = confy::load("d", None).unwrap();
    let input_arguements = InputArguements::parse();
    if input_arguements.verbose {
        println!("DEBUG: `main()` called. Arguements: {input_arguements:?}");
    }

    match input_arguements.command {
        Some(ValidSubcommands::NotesSubcommand(args)) => {
            handle_notes_subcommand(args, input_arguements.verbose)?;
        }
        None => (),
    }

    Ok(())
}

// TODO: Implement real logging
fn handle_notes_subcommand(args: NotesSubcommand, verbose_logging_enabled: bool) -> Result<()> {
    if verbose_logging_enabled {
        println!("DEBUG: `handle_notes_subcommand()` called. Arguements: {args:?}");
    }

    let project_path = "$HOME/notes";

    if args.edit {
        let editor_command = &fetch_editor_environment_variable(verbose_logging_enabled).unwrap();
        spawn_tmux_session(
            "NOTES TEST", 
            "Editor", 
            &format!("cd \"{project_path}\"; {editor_command}"), 
            verbose_logging_enabled
        )
        .unwrap_or_else(|_| panic!("Could not spwn tmux session"));
    }

    if args.git {
        let git_command = "lazygit";
        spawn_tmux_session(
            "NOTES TEST", 
            "Git", 
            &format!("cd \"{project_path}\"; {git_command}"), 
            verbose_logging_enabled
        )
        .unwrap_or_else(|_| panic!("Could not spwn tmux session"));
    }

    if args.terminal {
        let shell_command = &fetch_shell_environment_variable(verbose_logging_enabled).unwrap();
        spawn_tmux_session(
            "NOTES TEST", 
            "Terminal",
            &format!("cd \"{project_path}\"; {shell_command}"),
            verbose_logging_enabled
        )
        .unwrap_or_else(|_| panic!("Could not spwn tmux session"));
    }

    Ok(())
}

fn spawn_tmux_session(session_name: &str, window_name: &str, startup_command: &str, verbose_logging_enabled: bool) -> Result<()> {
    // TODO: Also check if window is already running?
    if check_if_tmux_session_is_already_spawned(session_name, verbose_logging_enabled).unwrap() {
        spawn_tmux_window(session_name, window_name, startup_command, verbose_logging_enabled);
        return Ok(())
    }

    if verbose_logging_enabled {
        println!("DEBUG: Creating a tmux session.");
    }

    Command::new("tmux")
        // `-L` == socket
        .args(["-L", &fetch_tmux_socket_name_environment_variable(verbose_logging_enabled).unwrap()]) // TODO: ???
        .args(["new-session"])
        .args(["-d"]) // TODO: ???
        .args(["-s", session_name])
        .args(["-n", window_name])
        .args([startup_command])
        .stdin(Stdio::inherit()) // TODO: ???
        .output() // TODO: ???
        .unwrap_or_else(|_| panic!("Failed to execute the tmux command!"));
    Ok(())
}

fn check_if_tmux_session_is_already_spawned(session_name: &str, verbose_logging_enabled: bool) -> Result<bool> {
    if verbose_logging_enabled {
        println!("DEBUG: Checking if window is already spawned.");
    }

    let result = Command::new("tmux")
        // `-L` == socket
        .args(["-L", &fetch_tmux_socket_name_environment_variable(verbose_logging_enabled).unwrap()]) // TODO: ???
        .args(["has-session"])
        .args(["-t", session_name])
        .output()?
        .status
        .success();

    if verbose_logging_enabled {
        println!("DEBUG: Checked if session is already spawned. Result: {result}");
    }

    Ok(result)
}

fn spawn_tmux_window(session_name: &str, window_name: &str, startup_command: &str, verbose_logging_enabled: bool) -> Result<()> {
    if verbose_logging_enabled {
        println!("DEBUG: Creating a tmux window.");
    }

    Command::new("tmux")
        .args(["-L", &fetch_tmux_socket_name_environment_variable(verbose_logging_enabled).unwrap()]) // TODO: ???
        .args(["new-window"])
        .args(["-d"]) // TODO: ???
        .args(["-n", window_name])
        .args(["-t", session_name])
        .args([startup_command])
        .stdin(Stdio::inherit()) // TODO: ???
        .output() // TODO: ???
        .unwrap_or_else(|_| panic!("Failed to execute the tmux command!"));
    Ok(())
}

fn fetch_editor_environment_variable(verbose_logging_enabled: bool) -> Result<String> {
    if verbose_logging_enabled {
        println!("DEBUG: `Checking $EDITOR` value.");
    }

    let editor = env::var("EDITOR").unwrap_or_else(|_| panic!("Editor not found."));
    if verbose_logging_enabled {
        println!("DEBUG: `$EDITOR` value checked. Value: {editor:?}");
    }

    Ok(editor)
}

fn fetch_shell_environment_variable(verbose_logging_enabled: bool) -> Result<String> {
    if verbose_logging_enabled {
        println!("DEBUG: `Checking $SHELL` value.");
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| panic!("Shell not found."));
    if verbose_logging_enabled {
        println!("DEBUG: `$Shell` value checked. Value: {shell:?}");
    }

    Ok(shell)
}

fn fetch_tmux_socket_name_environment_variable(verbose_logging_enabled: bool) -> Result<String> {
    // TODO: Make this generic and use crate application name as the prefix
    if verbose_logging_enabled {
        println!("DEBUG: `Checking $D_TMUX_SOCKET` value.");
    }

    let socket_name = env::var("D_TMUX_SOCKET")
        .ok()
        .unwrap_or(String::from("default"));

    if verbose_logging_enabled {
        println!("DEBUG: `$D_TMUX_SOCKET` value checked. Value: {socket_name:?}");
    }

    Ok(socket_name)
}
