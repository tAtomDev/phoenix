mod command_handler;
mod commands;
mod config;
mod discord;
mod event_handler;
mod prelude;
mod util;

use database::Database;
use event_handler::EventHandler;
use futures_util::StreamExt;
use std::sync::Arc;
use twilight_gateway::{
    cluster::{ClusterBuilder, ShardScheme},
    Event, Intents,
};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

use prelude::DynamicError;

#[tokio::main]
async fn main() -> Result<(), DynamicError> {
    dotenv::dotenv().ok();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let token = std::env::var("DISCORD_TOKEN_CANARY").expect("Discord token not found");

    let scheme = ShardScheme::Range {
        from: 0,
        to: 0,
        total: 1,
    };

    let intents = Intents::GUILDS;

    let (cluster, mut events) = ClusterBuilder::new(token.clone(), intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    let cluster = Arc::new(cluster);

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    let http = Arc::new(HttpClient::new(token));

    let database = Arc::new(
        Database::new(
            std::env::var("MONGODB_URI").unwrap(),
            config::CANARY.then(|| "testData").unwrap_or("data"),
        )
        .await,
    );

    let standby = Arc::new(Standby::new());

    let command_handler = Arc::new(command_handler::CommandHandler {
        database,
        standby: standby.clone(),
    });

    let handler = Arc::new(EventHandler::new(http, command_handler));

    let standby = standby.clone();
    while let Some((_, event)) = events.next().await {
        standby.process(&event);

        tokio::spawn(handle_event(event, handler.clone()));
    }

    Ok(())
}

async fn handle_event(event: Event, handler: Arc<EventHandler>) -> Result<(), DynamicError> {
    match event {
        Event::Ready(ready) => handler.ready(ready).await?,
        Event::ShardConnected(connected) => handler.shard_connected(connected).await?,
        Event::InteractionCreate(interaction) => handler.interaction_create(interaction).await?,
        _ => {}
    };

    Ok(())
}
