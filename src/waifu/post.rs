use anime_grubber::{
    agent::Agent,
    agents::waifu_pics::{Categories, Waifu},
};
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};
use shuller::random_usize;
use tracing::instrument;

use crate::{phrazes::PHRASES, Context};

/// Get 30 images
#[instrument]
async fn fetch_many(categorie: Categories) -> Result<Vec<String>, anime_grubber::error::Error> {
    let vaifu = Waifu::new(categorie);
    let images = vaifu.get_many().await?;
    Ok(images)
}
/// Get 1 random
#[instrument]
async fn fetch(categorie: &Categories) -> Result<String, anime_grubber::error::Error> {
    let vaifu = Waifu::new(categorie.clone());
    let images = vaifu.get().await?;
    Ok(images)
}
#[instrument]
async fn generate_post(
    url: &str,
    user: &str,
    user_avatar: &str,
    categorie: &str,
) -> Result<CreateEmbed, anime_grubber::error::Error> {
    Ok(CreateEmbed::default()
        .image(url)
        .url(url)
        .title(PHRASES[random_usize!(PHRASES.len())])
        .color(Colour::from_rgb(
            random_usize!(255) as u8,
            random_usize!(255) as u8,
            random_usize!(255) as u8,
        ))
        .description(format!(
            "
            Search category: {}
            ***Powered by [waifu.pics](https://waifu.pics/)***
            ",
            categorie
        ))
        .footer(CreateEmbedFooter::new(format!("shared by: {}", user)).icon_url(user_avatar)))
}
#[instrument(skip(ctx))]
pub async fn generate(
    many: bool,
    categorie: &Categories,
    ctx: &Context<'_>,
) -> Result<Vec<CreateEmbed>, anime_grubber::error::Error> {
    let categorie_str = std::convert::Into::<&str>::into(categorie);
    let user = ctx.author().name.as_str();
    let user_avatar = ctx.author().avatar_url().unwrap_or_default();
    if !many {
        let image = fetch(categorie).await?;
        Ok(vec![
            generate_post(&image, user, &user_avatar, categorie_str).await?,
        ])
    } else {
        let mut vec = vec![];
        let images = fetch_many(categorie.clone()).await?;
        for url in images.iter().take(3) {
            vec.push(generate_post(url, user, &user_avatar, categorie_str).await?)
        }
        Ok(vec)
    }
}
