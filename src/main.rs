use aws_client::Lambda;
use aws_client::S3;
use aws_types::SdkConfig;
use clap::Parser;

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
    let functions = lambda.list_functions().await;
    for function in functions {
        println!("{}", function.function_name().unwrap());
    }
}
