use std::sync::Arc;

use dsh_mqtt_client::{
    config::DshConfig,
    model::token_request_attr::RetrieveTokenRequest,
    service::{AuthenticationService, DshAuthenticationServiceAdapter},
};

#[tokio::main]
async fn main() {
    let dsh_conf = Arc::new(DshConfig {
        // TODO: create enum for env urls
        rest_token_endpoint: "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token".to_string(),
        mqtt_token_endpoint: "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token"
            .to_string(),
    });

    let retrieve_request = RetrieveTokenRequest {
        tenant: "tenant-name".to_string(), // change here before commit
        //TODO:  initiate from ENV VAR
        api_key: "********".to_string(), // change here before commit
        claims: None,                    //better example
        client_id: uuid::Uuid::new_v4().to_string(),
    };
    let service = DshAuthenticationServiceAdapter::new(dsh_conf);
    let mqtt_token = service
        .retrieve_token(retrieve_request.clone())
        .await
        .unwrap();

    print!("mqtt -> {:?}", mqtt_token);

    //TODO: another file example for stream
}
