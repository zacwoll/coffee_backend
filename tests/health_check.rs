use coffee_backend;

#[tokio::test]
async fn dummy_test() -> Result<(), std::io::Error> {
    coffee_backend::run().await?;
	Ok(())
}