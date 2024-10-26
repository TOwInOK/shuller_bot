use ::serenity::all::ActivityData;
use boop::boop;
use help::help;
use poise::serenity_prelude as serenity;
use rule34::image_board::porno;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use url::Url;
use waifu::tab::{actions, characters, emotions, waifu_nsfw};
mod boop;
mod help;
pub mod phrazes;
mod rule34;
mod waifu;
// Структура данных, доступная во всех командах
pub struct Data {}

// Определение типа ошибок для команд
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

// Main

#[tokio::main]
async fn main() {
    let token = std::env::var("DS_TOKEN").expect("missing DISCORD_TOKEN");
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .unwrap();
    let intents = serenity::GatewayIntents::MESSAGE_CONTENT;

    // Создаем фреймворк poise
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                porno(),
                boop(),
                characters(),
                emotions(),
                actions(),
                waifu_nsfw(),
                help(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(">".into()),
                mention_as_prefix: true,
                ..Default::default()
            },

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
            name: "LuuMa".to_string(),
            kind: serenity::ActivityType::Playing,
            state: Some("🎴 Картишки казино, 🎰 три слота, ↗️ мы на подъёме".to_string()),
            url: Some(
                Url::parse("https://github.com/TOwInOK/shuller_bot")
                    .expect("Fail to build url for activity"),
            ),
        })
        .await;
    client
        .expect("Fail to build client")
        .start_autosharded()
        .await
        .expect("Fail to build client");
}
