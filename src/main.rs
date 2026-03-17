use bevy_screeps_lua::app::create_app;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting bevy_screeps_lua...");

    let mut app = create_app();
    app.run();

    Ok(())
}