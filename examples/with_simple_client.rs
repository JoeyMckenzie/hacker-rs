use hacker_rs::{client::HackerNewsClient, errors::HackerNewsClientError};

#[tokio::main]
async fn main() -> Result<(), HackerNewsClientError> {
    // Build your client at the start of your application process
    let client = HackerNewsClient::new();

    // Optionally build your client with a configured request timeout
    let _client_with_timeout = HackerNewsClient::new_with_timeout(2);

    // Call various endpoints with your client instance
    let unknown_item = client.get_item(69).await?;
    dbg!(&unknown_item);

    // Determine what the item type is
    let item_type = unknown_item.get_item_type();
    dbg!(item_type);

    // Check if the item is job
    assert!(unknown_item.is_story());

    // Conveniently request typed items for known IDs
    let story_item = client.get_story(69).await?;
    dbg!(&story_item);

    // Get child comments associated to the story
    let comment = client
        .get_item(*story_item.comments.first().unwrap())
        .await?;

    // Retrieve user information
    let user = client.get_user("joeymckenzie").await?;
    dbg!(user);

    Ok(())
}
