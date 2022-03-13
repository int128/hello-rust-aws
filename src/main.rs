#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&shared_config);

    match show_instances(&client).await {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
    }
}

async fn show_instances(client: &aws_sdk_ec2::Client) -> Result<(), aws_sdk_ec2::Error> {
    let instances = list_instances(client).await?;
    for i in instances {
        println!("instance ID = {}", i)
    }
    Ok(())
}

async fn list_instances(client: &aws_sdk_ec2::Client) -> Result<std::vec::Vec<String>, aws_sdk_ec2::Error> {
    let describe_instances = client.
        describe_instances().
        send().
        await?;

    let reservations = describe_instances.reservations.unwrap_or_default();

    let instances = reservations.iter().flat_map(|r|
        r.instances.as_ref().unwrap().iter().flat_map(|i|
            i.instance_id.clone()
        ));

    Ok(instances.collect())
}
