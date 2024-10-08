use bevy::prelude::*;
use bevy_headless_console::{
    BasicTerminalPlugin, ConsoleCommandEntered, ConsolePlugin, ConsoleSet,
};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ConsolePlugin, BasicTerminalPlugin))
        .add_systems(Update, raw_commands.in_set(ConsoleSet::Commands))
        .run();
}

fn raw_commands(mut console_commands: EventReader<ConsoleCommandEntered>) {
    for ConsoleCommandEntered { command_name, args } in console_commands.read() {
        println!(r#"Entered command "{command_name}" with args {:#?}"#, args);
    }
}
