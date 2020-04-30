mod shipthing {
    pub mod v1 {
        #![allow(clippy::all)]
        #![allow(clippy::pedantic)]

        tonic::include_proto!("shipthing.v1");
    }
}

const SERVER_URI: &str = "http://localhost:5901";
const SECRET_FILE_NAME: &str = "secret";

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    pretty_env_logger::init();

    let mut client = shipthing::v1::player_api_client::PlayerApiClient::connect(SERVER_URI).await?;
    log::info!("connected to: {:?}", SERVER_URI);

    let secret = load_secret().await?;

    let join_response = client
        .join(shipthing::v1::JoinRequest {
            name: env!("USER").to_owned(),
            secret: secret.as_bytes().to_vec(),
        })
        .await?;
    let session_id = join_response.into_inner().session_id;
    log::info!("session_id: {:x?}", hex::encode(session_id));

    Ok(())
}

async fn load_secret() -> Result<uuid::Uuid, failure::Error> {
    use tokio::fs;
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;

    match fs::File::open(SECRET_FILE_NAME).await {
        Ok(mut file) => {
            let mut data = Vec::with_capacity(16);
            file.read_to_end(&mut data).await?;
            Ok(uuid::Uuid::from_slice(&data)?)
        }
        Err(_) => {
            let secret = uuid::Uuid::new_v4();
            let mut file = fs::File::create(SECRET_FILE_NAME).await?;
            file.write_all(secret.as_bytes()).await?;
            Ok(secret)
        }
    }
}
