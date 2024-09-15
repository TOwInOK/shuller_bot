use crate::{phrazes::PHRASES, Context};
use ::serenity::all::{CreateMessage, ReactionType};
use poise::{serenity_prelude as serenity, CreateReply};
use shuller::{prelude::Post, random_usize};
use tracing::debug;

pub async fn paginate(ctx: Context<'_>, posts: &[Post]) -> Result<(), crate::Error> {
    let ctx_id = ctx.id();
    let mut current_page = 0;
    let buttons = |current_num: &usize| {
        let mut vec = vec![];
        if posts.len() != 1 {
            posts.iter().enumerate().for_each(|(num, _)| {
                vec.push(if current_num == &num {
                    serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                        .label(format!("{}", num + 1))
                        .style(serenity::ButtonStyle::Success)
                } else {
                    serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                        .label(format!("{}", num + 1))
                        .style(serenity::ButtonStyle::Secondary)
                })
            })
        } else {
            vec.push(
                serenity::CreateButton::new(format!("{}-{}", ctx_id, 0))
                    .label(format!("{}", posts[0].id))
                    .style(serenity::ButtonStyle::Success),
            );
        }
        vec.push(
            serenity::CreateButton::new(format!("{}-send_to_chat", ctx_id))
                .emoji(ReactionType::Unicode("🚀".to_string()))
                .style(serenity::ButtonStyle::Primary),
        );
        vec
    };
    let post = |page: &usize, shared: bool| {
        debug!("page is: {:#?}", &posts[*page]);
        match shared {
            true => serenity::CreateEmbed::default()
                .image(&posts[*page].sample_url)
                .url(&posts[*page].file_url)
                .title(PHRASES[random_usize!(PHRASES.len())])
                .color(serenity::Colour::from_rgb(
                    random_usize!(255) as u8,
                    random_usize!(255) as u8,
                    random_usize!(255) as u8,
                ))
                .description(format!(
                    "
                    [Post](https://rule34.xxx/index.php?page=post&s=view&id={})
                    [Tag Link](https://rule34.xxx/index.php?page=post&s=list&tags={})
                    **Tags:**
                    ```{}```
                    ",
                    &posts[*page].id,
                    &posts[*page]
                        .tags
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join("+"),
                    &posts[*page].tags,
                ))
                .author(serenity::CreateEmbedAuthor::new(&posts[*page].owner))
                .footer(
                    serenity::CreateEmbedFooter::new(format!("shared by: {}", ctx.author().name))
                        .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                ),
            false => serenity::CreateEmbed::default()
                .image(&posts[*page].sample_url)
                .url(&posts[*page].file_url)
                .title(PHRASES[random_usize!(PHRASES.len())])
                .color(serenity::Colour::from_rgb(
                    random_usize!(255) as u8,
                    random_usize!(255) as u8,
                    random_usize!(255) as u8,
                ))
                .description(format!(
                    "
                    [Post](https://rule34.xxx/index.php?page=post&s=view&id={})
                    [Tag Link](https://rule34.xxx/index.php?page=post&s=list&tags={})
                    **Tags:**
                    ```{}```
                    ",
                    &posts[*page].id,
                    &posts[*page]
                        .tags
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join("+"),
                    &posts[*page].tags,
                ))
                .author(serenity::CreateEmbedAuthor::new(&posts[*page].owner)),
        }
    };
    let reply = {
        let components = serenity::CreateActionRow::Buttons(buttons(&current_page));

        CreateReply::default()
            .embed(post(&current_page, false))
            .components(vec![components])
    };

    ctx.send(reply).await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60 * 60))
        .await
    {
        if press.data.custom_id == format!("{}-send_to_chat", ctx_id) {
            ctx.channel_id()
                .send_message(
                    &ctx.http(),
                    CreateMessage::new().embed(post(&current_page, true)),
                )
                .await?;
        } else {
            current_page = {
                let temp: Vec<&str> = press.data.custom_id.split("-").collect();
                temp[1].parse().unwrap_or(0)
            };
        }
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(post(&current_page, false))
                        .components(vec![serenity::CreateActionRow::Buttons(buttons(
                            &current_page,
                        ))]),
                ),
            )
            .await?;
    }

    Ok(())
}
