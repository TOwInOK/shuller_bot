use crate::{phrazes::PHRAZES, Context};
// use phrazes;
use poise::{serenity_prelude as serenity, CreateReply};
use shuller::{prelude::Post, random_usize};
use tracing::info;

pub async fn paginate(ctx: Context<'_>, posts: &[Post]) -> Result<(), crate::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let mut current_page = 0;
    let buttons = |x: &usize| {
        let mut vec = vec![];
        if posts.len() != 1 {
            for num in 0..posts.len() {
                vec.push(if x == &num {
                    serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                        .label(format!("{}", num + 1))
                        .style(serenity::ButtonStyle::Success)
                } else {
                    serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                        .label(format!("{}", num + 1))
                        .style(serenity::ButtonStyle::Secondary)
                })
            }
        } else {
            vec.push(
                serenity::CreateButton::new(format!("{}-{}", ctx_id, 0))
                    .label(format!("{}", posts[0].id))
                    .style(serenity::ButtonStyle::Success),
            );
        }
        vec
    };

    let post = |page: &usize| {
        info!("page is: {:#?}", &posts[*page]);
        serenity::CreateEmbed::default()
            .image(&posts[*page].sample_url)
            .url(&posts[*page].file_url)
            .title(PHRAZES[random_usize!(PHRAZES.len())])
            .color(serenity::Colour::from_rgb(
                random_usize!(255) as u8,
                random_usize!(255) as u8,
                random_usize!(255) as u8,
            ))
            .description(format!(
                "**Tags:**
            {}",
                &posts[*page].tags
            ))
            .author(serenity::CreateEmbedAuthor::new(&posts[*page].owner))
    };
    // Send the embed with the first page as content
    let reply = {
        let components = serenity::CreateActionRow::Buttons(buttons(&current_page));

        CreateReply::default()
            .embed(post(&current_page))
            .components(vec![components])
    };

    ctx.send(reply).await?;

    // Loop through incoming interactions with the navigation buttons
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 1 hours
        .timeout(std::time::Duration::from_secs(3600))
        .await
    {
        // Depending on which button was pressed, go to next or previous page
        current_page = {
            let temp: Vec<&str> = press.data.custom_id.split("-").collect();
            temp[1].parse().unwrap_or(0)
        };
        // Update the message with the new page contents
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(post(&current_page))
                        .components(vec![serenity::CreateActionRow::Buttons(buttons(
                            &current_page,
                        ))]),
                ),
            )
            .await?;
    }

    Ok(())
}
