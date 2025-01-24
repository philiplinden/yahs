use bevy::prelude::*;
use bevy_console::{
    AddConsoleCommand, ConsoleConfiguration, ConsoleOpen, ConsolePlugin,
};
use yahs_cli::{
    start_command, get_command, set_command,
    StartCommand, GetCommand, SetCommand,
};


const OPEN_CONSOLE_BY_DEFAULT: bool = false;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(
        ConsolePlugin,
    )
    .insert_resource(ConsoleConfiguration {
        top_pos: 0.0,
        left_pos: 0.0,
        height: 300.0,
        width: 1280.0,
        show_title_bar: false,
        ..Default::default()
    })
    .insert_resource(ConsoleOpen {
        open: OPEN_CONSOLE_BY_DEFAULT,
    })
    .add_console_command::<StartCommand, _>(start_command)
    .add_console_command::<GetCommand, _>(get_command)
    .add_console_command::<SetCommand, _>(set_command);
}
