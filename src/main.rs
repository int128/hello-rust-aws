#[tokio::main]
async fn main() {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_ec2::Client::new(&shared_config);

    let describe_instances = client.
        describe_instances().
        send().
        await.
        unwrap();

    for r in describe_instances.reservations.unwrap_or_default() {
        for i in r.instances.unwrap_or_default() {
            let instance_id = i.instance_id.unwrap();
            println!("instance_id={}", instance_id)
        }
    }
}
