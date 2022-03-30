use aws_client::Lambda;
use aws_client::S3;
use aws_types::SdkConfig;
use clap::Parser;

use aws_sdk_lambda::model::{Concurrency, FunctionCodeLocation, FunctionConfiguration};
use aws_sdk_lambda::output::GetFunctionOutput;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    profile: Option<String>,
}

async fn config(profile: Option<String>) -> SdkConfig {
    match profile {
        Some(profile) => {
            let cp = aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name(profile)
                .build();
            aws_config::from_env().credentials_provider(cp).load().await
        }
        None => aws_config::load_from_env().await,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = config(args.profile).await;

    let s3 = S3::new(&config);
    let buckets = s3.list_buckets().await;
    for bucket in buckets {
        println!("{}", bucket.name().unwrap());
    }

    let lambda = Lambda::new(&config);
    let funcs = lambda.list_functions().await;
    for func in funcs {
        let func_name = func.function_name().unwrap();
        let func: GetFunctionOutput = lambda.get_function(func_name).await.unwrap();
        let configuration: Option<&FunctionConfiguration> = func.configuration();
        if let Some(configuration) = configuration {
            println!("{}", configuration.function_name().unwrap());
        }
        let code: Option<&FunctionCodeLocation> = func.code();
        if let Some(code) = code {
            println!("repository_type: {}", code.repository_type().unwrap());
        }
        if let Some(tags) = func.tags() {
            println!("tags:\t");
            for (key, value) in tags {
                println!("{}: {}", key, value);
            }
        }
        let concurrency: Option<&Concurrency> = func.concurrency();
        if let Some(concurrency) = concurrency {
            if let Some(n) = concurrency.reserved_concurrent_executions() {
                println!("reserved_concurrent_executions: {}", n);
            }
        }
    }
}
