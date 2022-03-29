use aws_sdk_lambda::model::FunctionConfiguration;
use aws_sdk_lambda::Client;
use aws_types::SdkConfig;

pub struct Lambda {
    client: Client,
}

impl Lambda {
    pub fn new(config: &SdkConfig) -> Lambda {
        Lambda {
            client: Client::new(config),
        }
    }

    pub async fn list_functions(&self) -> Vec<FunctionConfiguration> {
        let resp = self.client.list_functions().send().await.unwrap();
        resp.functions().unwrap().to_vec()
    }
}
