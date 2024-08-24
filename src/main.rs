use ::serenity::all::ActivityData;
use boop::boop;
use image_board::porno;
use poise::serenity_prelude as serenity;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use url::Url;
mod boop;
mod image_board;
pub mod paginate;
pub mod phrazes;
// Структура данных, доступная во всех командах
pub struct Data {}

// Определение типа ошибок для команд
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

// Main

#[tokio::main]
async fn main() {
    let token = std::env::var("DS_TOCKEN").expect("missing DISCORD_TOKEN");
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .unwrap();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Создаем фреймворк poise
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![porno(), boop()],

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    // Создаем и запускаем клиента
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(ActivityData {
            name: "Проповедничает Shuller".to_string(),
            kind: serenity::ActivityType::Custom,
            state: Some("Едет в kfc, жарить кур".to_string()),
            url: Some(Url::parse("https://crates.io/crates/shuller").unwrap()),
        })
        .await;
    client.unwrap().start_autosharded().await.unwrap();
}
