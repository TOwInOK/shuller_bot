use crate::{paginate, Context, Error};
use shuller::prelude::*;
use tracing::info;

static LEN: usize = 4;
static LEN_MAX: usize = 30;

/// Get your loved pngs
#[poise::command(
    slash_command,
    guild_only,
    nsfw_only,
    description_localized("ru", "Получить дозу эмоций"),
    // name_localized("ru", "порно"),
    ephemeral
)]
pub async fn porno(
    ctx: Context<'_>,
    #[description = "let me your favorite tags!"]
    #[description_localized("ru", "Дай мне свои любимые теги")]
    // #[name_localized("ru", "позитивные теги")]
    // #[name_localized("us", "positive tags")]
    positive_tags: Option<String>,
    #[description = "let me your unfavorite tags!"]
    #[description_localized("ru", "Что не нравится?")]
    // #[name_localized("ru", "негативные теги")]
    // #[name_localized("us", "negative tags")]
    negative_tags: Option<String>,
    #[description = "Do you wana some special?!"]
    #[description_localized("ru", "Позвони и узнай, как там с деньгами!")]
    // #[name_localized("ru", "пробив по номеру")]
    id: Option<usize>,
    #[description = "How many do you want?"]
    #[description_localized("ru", "Сколько выдать?")]
    // #[name_localized("ru", "количество")]
    size: Option<usize>,
) -> Result<(), Error> {
    let mut posts = vec![];
    let size = size.unwrap_or(3);
    if size > LEN {
        return Err("**Error: TOOO BIG**, max size is 4".into());
    }
    if let Some(id) = id {
        info!("found some id: {}", &id);
        let mut data = match R34!(D; id = id) {
            Ok(e) => e.data(),
            Err(_) => return Err("**Error: Posts** not found".into()),
        };
        posts.append(&mut data);
    } else if positive_tags.is_some() || negative_tags.is_some() {
        info!(
            "found some tags: p = {:#?}, n = {:#?}",
            &positive_tags, &negative_tags
        );
        let a = R34!(D;
            p = positive_tags.unwrap_or("".to_string()).split_whitespace().collect(),
            // p = ["ai_generated"].to_vec(),
                n = negative_tags.unwrap_or("".to_string()).split_whitespace().collect(),
                // n = [].to_vec(),
                limit = LEN_MAX as u16
        );
        match a {
            Ok(r) => {
                let mut a = r.shuffle().data();
                posts.append(&mut a);
            }
            Err(e) => return Err(format!("**Error: Posts** not found\nErr{}", e).into()),
        }
    } else {
        info!("Nothing addition params found");
        for _ in 0..size {
            match R34!(R; D) {
                Ok(data) => posts.push(data.data()[0].clone()),
                Err(_) => continue,
            }
        }
    }

    if posts.is_empty() {
        return Err("**Error: Posts** not found".into());
    }
    paginate::paginate(
        ctx,
        if posts.len() < 4 {
            &posts[..posts.len()]
        } else {
            &posts[..4]
        },
    )
    .await?;

    Ok(())
}
