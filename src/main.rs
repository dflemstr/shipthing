mod config;
mod handler;
mod main_loop;
mod proto;
mod renderer;
mod server;
mod state;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    use futures::lock;
    use std::sync;

    pretty_env_logger::init();

    let world = sync::Arc::new(lock::Mutex::new(state::State::new()));

    tokio::spawn(server::run(sync::Arc::clone(&world)));
    tokio::spawn(main_loop::run(sync::Arc::clone(&world)));
    renderer::run(sync::Arc::clone(&world)).await?;

    Ok(())
}
