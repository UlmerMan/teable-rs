use teable::client::TeableClient;
use teable::models::base::{PostBaseRequest, UpdateBaseRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TEABLE_TOKEN").expect("TEABLE_TOKEN environment variable not set");
    let space_id =
        std::env::var("TEABLE_SPACE_ID").expect("TEABLE_SPACE_ID environment variable not set");

    let client = TeableClient::builder().token(token).build()?;

    println!("Creating base...");

    let request = PostBaseRequest {
        space_id,
        name: "Rust SDK Test Base".to_string(),
        description: Some("Created by teable-rs example".to_string()),
        icon: Some("🦀".to_string()),
        template: None,
    };

    let base = client.bases().post(&request).await?;

    println!("Created base:");
    println!("  id:   {}", base.id);
    println!("  name: {}", base.name);

    println!();
    println!("Fetching base...");

    let fetched = client.bases().get(&base.id).await?;

    println!("Fetched base:");
    println!("  id:   {}", fetched.id);
    println!("  name: {}", fetched.name);

    assert_eq!(fetched.id, base.id);
    assert_eq!(fetched.name, base.name);

    println!();
    println!("Updating base...");

    let updated = client
        .bases()
        .patch(
            &base.id,
            &UpdateBaseRequest {
                name: Some("Rust SDK Updated Base".to_string()),
                icon: Some("rocket".to_string()),
            },
        )
        .await?;

    println!("Updated base:");
    println!("  name: {}", updated.name);

    if let Ok(anchor_id) = std::env::var("TEABLE_ANCHOR_BASE_ID") {
        println!();
        println!("Updating base order...");

        client
            .bases()
            .update_order(
                &base.id,
                &teable::models::base::order::Order {
                    anchor_id,
                    position: teable::models::base::order::Position::After,
                },
            )
            .await?;

        println!("Base order updated successfully.");
    }

    println!();
    println!("Deleting base...");

    client.bases().delete(&base.id).await?;

    println!("Base deleted successfully.");

    Ok(())
}
