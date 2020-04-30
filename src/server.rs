use crate::handler;
use crate::proto;
use crate::state;
use futures::lock;
use std::sync;

pub async fn run(state: sync::Arc<lock::Mutex<state::State>>) -> Result<(), failure::Error> {
    tonic::transport::Server::builder()
        .add_service(
            proto::shipthing::v1::player_api_server::PlayerApiServer::new(handler::Handler::new(
                state,
            )),
        )
        .serve("[::1]:5901".parse()?)
        .await?;
    Ok(())
}
