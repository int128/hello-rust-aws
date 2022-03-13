#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&shared_config);
    show_instances(&client).await
}

async fn show_instances(client: &aws_sdk_ec2::Client) {
    let describe_instances = client.
        describe_instances().
        send().
        await.
        unwrap();

    let reservations = describe_instances.reservations.unwrap_or_default();

    let instances = reservations.iter().flat_map(|r| {
        r.instances.as_ref().unwrap().iter().flat_map(|i| &i.instance_id)
    });

    for i in instances {
        println!("owner_id = {}", i)
    }
}
