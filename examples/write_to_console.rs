use bevy::prelude::*;
use bevy_headless_console::{BasicTerminalPlugin, ConsolePlugin, ConsoleSet, PrintConsoleLine};

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ConsolePlugin, BasicTerminalPlugin))
        // NOTE: this wouldn't work for this particular case,
        // systems in the [`ConsoleSet::Commands`] do not run if there are no console commands entered
        // .add_systems(Update, write_to_console.in_set(ConsoleSet::Commands))
        // the below is the equivalent but without run conditions
        .add_systems(Update, write_to_console.after(ConsoleSet::ConsoleInput))
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello".into()));
}
