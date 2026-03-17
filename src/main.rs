use bevy_screeps_lua::app::create_app;

fn main() -> anyhow::Result<()> {
    let mut app = create_app();
    app.run();

    Ok(())
}