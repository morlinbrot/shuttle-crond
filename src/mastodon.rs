use mastodon_async::helpers::env::from_env;
use mastodon_async::helpers::toml;
use mastodon_async::{helpers::cli, Mastodon, Result};
use mastodon_async::{Data, Registration};

pub async fn connect() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load env vars");

    let data = from_env().expect("Failed to parse data from env vars");

    let mastodon = Mastodon::from(data);

    let you = mastodon.verify_credentials().await?;

    println!("You are: {:#?}", you);

    Ok(())
}
