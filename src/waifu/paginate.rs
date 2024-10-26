use crate::Context;
use ::serenity::all::UserId;
use anime_grubber::waifu_pics::Categories;
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::all::{CreateEmbed, CreateMessage, ReactionType};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::instrument;

// Глобальное хранилище времени последнего обновления для каждого пользователя
static LAST_REFRESH: LazyLock<Mutex<HashMap<UserId, u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Получает текущее время в секундах с начала эпохи UNIX
fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Обновляет время последнего обновления для конкретного пользователя
fn update_refresh_time(user_id: UserId, time: u64) {
    let mut map = LAST_REFRESH.lock().unwrap();
    map.insert(user_id, time);
}

/// Проверяет, должна ли кнопка обновления быть отключена
/// (true если прошло менее 3 секунд с последнего обновления)
fn is_refresh_disabled(user_id: UserId, current_time: u64) -> bool {
    let map = LAST_REFRESH.lock().unwrap();
    let last_refresh = map.get(&user_id).copied().unwrap_or(0);
    current_time - last_refresh < 3
}

/// Создает набор кнопок управления для сообщения
fn create_buttons(
    ctx_id: u64,
    current_page: &usize,
    embeds_len: usize,
    is_disabled: bool,
) -> Vec<serenity::CreateButton> {
    let mut buttons = vec![];

    // Добавляем кнопки навигации по страницам, если их больше одной
    if embeds_len != 1 {
        for num in 0..embeds_len {
            let button = if current_page == &num {
                // Текущая страница выделяется зеленым
                serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                    .label(format!("{}", num + 1))
                    .style(serenity::ButtonStyle::Success)
            } else {
                // Остальные страницы серые
                serenity::CreateButton::new(format!("{}-{}", ctx_id, num))
                    .label(format!("{}", num + 1))
                    .style(serenity::ButtonStyle::Secondary)
            };
            buttons.push(button);
        }
    }

    // Кнопка для отправки изображения в общий чат
    buttons.push(
        serenity::CreateButton::new(format!("{}-send_to_chat", ctx_id))
            .emoji(ReactionType::Unicode("🚀".to_string()))
            .style(serenity::ButtonStyle::Primary),
    );

    // Кнопка обновления контента
    buttons.push(
        serenity::CreateButton::new(format!("{}-refresh", ctx_id))
            .emoji(ReactionType::Unicode("🔄".to_string()))
            .style(serenity::ButtonStyle::Secondary)
            .disabled(!is_disabled),
    );

    buttons
}

/// Обработчик отправки изображения в общий чат
async fn handle_send_to_chat(ctx: &Context<'_>, embed: &CreateEmbed) -> Result<(), crate::Error> {
    let message = CreateMessage::new()
        .embed(embed.clone())
        .allowed_mentions(serenity::CreateAllowedMentions::new());

    ctx.channel_id().send_message(&ctx.http(), message).await?;
    Ok(())
}

async fn handle_refresh(
    ctx: &Context<'_>,
    press: &serenity::ComponentInteraction,
    items: &mut Vec<CreateEmbed>,
    categorie: &Categories,
    current_page: &mut usize,
) -> Result<(), crate::Error> {
    let now = get_current_time();
    update_refresh_time(ctx.author().id, now);

    let new_embeds = super::post::generate(true, categorie, ctx).await?;
    items.clear();
    items.extend(new_embeds.into_iter());
    *current_page = 0;

    let buttons = create_buttons(ctx.id(), current_page, items.len(), false);

    press
        .create_response(
            ctx.serenity_context(),
            serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::new()
                    .embed(items[*current_page].clone())
                    .components(vec![serenity::CreateActionRow::Buttons(buttons)])
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
/// Основная функция пагинации, создает интерактивное сообщение с кнопками
#[instrument(skip(ctx))]
pub async fn paginate(
    ctx: Context<'_>,
    embeds: &[CreateEmbed],
    categorie: Categories,
    run_now: bool,
) -> Result<(), crate::Error> {
    let ctx_id = ctx.id();
    let user_id = ctx.author().id;
    let now = get_current_time();
    let mut embeds = embeds.to_vec();
    // Инициализация состояния обновления
    update_refresh_time(user_id, now);
    let is_disabled = is_refresh_disabled(user_id, now);
    let mut current_page = 0;

    // Создаем начальное сообщение
    if run_now {
        let reply = CreateReply::default()
            .ephemeral(true)
            .embed(embeds[current_page].clone());
        ctx.send(reply).await?;
    } else {
        let buttons = create_buttons(ctx_id, &current_page, embeds.len(), is_disabled);
        let reply = CreateReply::default()
            .ephemeral(true)
            .embed(embeds[current_page].clone())
            .components(vec![serenity::CreateActionRow::Buttons(buttons)]);
        ctx.send(reply).await?;
    }

    // Начинаем слушать взаимодействия с кнопками
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60 * 60))
        .await
    {
        // Обрабатываем различные действия с кнопками
        if press.data.custom_id == format!("{}-send_to_chat", ctx_id) {
            handle_send_to_chat(&ctx, &embeds[current_page]).await?;
        } else if press.data.custom_id == format!("{}-refresh", ctx_id) {
            handle_refresh(&ctx, &press, &mut embeds, &categorie, &mut current_page).await?;
            continue;
        } else {
            // Обработка переключения страниц
            current_page = press
                .data
                .custom_id
                .split('-')
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }

        // Обновляем сообщение при переключении страниц
        if !press.data.custom_id.contains("refresh") {
            let buttons = create_buttons(ctx_id, &current_page, embeds.len(), is_disabled);
            press
                .create_response(
                    ctx.serenity_context(),
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::new()
                            .embed(embeds[current_page].clone())
                            .components(vec![serenity::CreateActionRow::Buttons(buttons)]),
                    ),
                )
                .await?;
        }
    }

    Ok(())
}
