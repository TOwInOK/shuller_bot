use ::serenity::all::ActivityData;
use poise::serenity_prelude as serenity;
use rand::rngs::OsRng;
use shuller::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

// Структура данных, доступная во всех командах
struct Data {
    rng: Arc<Mutex<OsRng>>,
}

// Определение типа ошибок для команд
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
const MAX_SIZE: usize = 5;
const LEN: usize = 30;

/// Give you random img url from rule34
#[poise::command(
    slash_command,
    guild_only,
    nsfw_only,
    description_localized("ru", "Давайте по чесноку, руки на стол")
)]
async fn porno(
    ctx: Context<'_>,
    #[description = "let me your favorite tags!"]
    #[description_localized("ru", "Покажи мне свои мягкие булочки")]
    #[name_localized("ru", "позитив")]
    positive_tags: Option<String>,
    #[description = "show me your secrets!"]
    #[description_localized("ru", "Нуус... поведуй что не нравится")]
    #[name_localized("ru", "негатив")]
    negative_tags: Option<String>,
    #[description = "Do you wana some special?!"]
    #[description_localized("ru", "Мне кажется ты знаешь что хочешь")]
    #[name_localized("ru", "знакомый")]
    id: Option<usize>,
    #[description = "How many do you want?"]
    #[description_localized("ru", "Хо-хо-хо, а вы мистер любите такой размер?!")]
    #[name_localized("ru", "размерчик")]
    size: Option<usize>,
) -> Result<(), Error> {
    if let Some(size) = size {
        if size > MAX_SIZE {
            return Err("Error: Your size is toooo big, max size = 5".into());
        }
    }
    if let Some(id) = id {
        send_image_by_id(ctx, id).await
    } else if positive_tags.is_some() || negative_tags.is_some() {
        send_images_by_tags(ctx, positive_tags, negative_tags, size).await
    } else {
        feel_luck(ctx, size).await
    }
}
#[inline]
async fn send_image_by_id(ctx: Context<'_>, id: usize) -> Result<(), Error> {
    let img = R34!(D; id = id).expect("No one img found");
    let url = img.get_f_url().expect("Failed to get image URL");
    ctx.say(url).await?;
    Ok(())
}

async fn send_images_by_tags(
    ctx: Context<'_>,
    positive_tags: Option<String>,
    negative_tags: Option<String>,
    size: Option<usize>,
) -> Result<(), Error> {
    let positive: Vec<&str> = positive_tags
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .collect();

    let negative: Vec<&str> = negative_tags
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .collect();

    let posts =
        R34!(D; p = positive, n = negative, limit = LEN as u16).expect("Failed to fetch posts");
    let posts = posts.get_f_urls();
    if posts.is_empty() {
        return Err("**Error: Nothing found**, try to cum on any tags!".into());
    }
    if let Some(size) = size {
        ctx.say(posts[0..size].join(" ")).await?;
    } else {
        let image = get_from_vec(ctx, &posts).await?;
        ctx.say(image).await?;
    }
    Ok(())
}
#[inline]
async fn feel_luck(ctx: Context<'_>, size: Option<usize>) -> Result<(), Error> {
    let mut _image_url = String::new();
    if let Some(size) = size {
        let pool = get_limited(&ctx.data().rng).await;
        _image_url = pool.get_f_urls()[0..size].join(" ");
    } else {
        _image_url = get_one(&ctx.data().rng).await;
    }
    ctx.say(_image_url).await?;
    Ok(())
}

#[inline]
async fn get_from_vec<'a>(ctx: Context<'_>, urls: &'a [&str]) -> Result<&'a str, Error> {
    let id = ctx.data().rng.lock().await.gen_range(0..urls.len());
    Ok(urls[id])
}
#[inline]
async fn get_one(rng: &Mutex<OsRng>) -> String {
    let id = rng.lock().await.gen_range(0..10900000);
    let porno = R34!(D; id = id).expect("Не парсятся твои посты!");
    porno.get_f_url().unwrap().to_string()
}
#[inline]
async fn get_limited(rng: &Mutex<OsRng>) -> Posts {
    let _id: usize = rng.lock().await.gen_range(0..10900000);
    R34!(D; ,limit = 5, page = _id).expect("Не парсятся твои посты!")
}

// Main

#[tokio::main]
async fn main() {
    let token = std::env::var("DS_TOCKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Создаем фреймворк poise
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![porno()],

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    rng: Arc::new(Mutex::new(OsRng)),
                })
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
