use crate::config;
use crate::proto::shipthing::v1 as proto;
use crate::state;
use futures::lock;
use std::sync;

#[derive(Clone, Debug)]
pub struct Handler {
    state: sync::Arc<lock::Mutex<state::State>>,
}

impl Handler {
    pub fn new(state: sync::Arc<lock::Mutex<state::State>>) -> Self {
        Handler { state }
    }
}

#[tonic::async_trait]
impl proto::player_api_server::PlayerApi for Handler {
    async fn get_config(
        &self,
        _: tonic::Request<proto::GetConfigRequest>,
    ) -> Result<tonic::Response<proto::GetConfigResponse>, tonic::Status> {
        Ok(tonic::Response::new(proto::GetConfigResponse {
            world_width: config::WORLD_WIDTH,
            world_height: config::WORLD_HEIGHT,
            energy_min_level: config::ENERGY_MIN_LEVEL,
            energy_max_level: config::ENERGY_MAX_LEVEL,
            energy_replenish_rate: config::ENERGY_REPLENISH_RATE,
            energy_boost_cost: config::ENERGY_BOOST_COST,
            ship_radius: config::SHIP_RADIUS,
            ship_initial_velocity: config::SHIP_INITIAL_VELOCITY,
        }))
    }

    async fn join(
        &self,
        request: tonic::Request<proto::JoinRequest>,
    ) -> Result<tonic::Response<proto::JoinResponse>, tonic::Status> {
        let request = request.into_inner();

        let uuid = uuid::Uuid::from_slice(&request.secret)
            .map_err(|err| tonic::Status::invalid_argument(format!("{}", err)))?;

        let session_id = self
            .state
            .lock()
            .await
            .join(request.name, state::Secret(uuid));

        let session_id = session_id.0.as_bytes().to_vec();

        Ok(tonic::Response::new(proto::JoinResponse { session_id }))
    }
}
