use crate::{
    waifu::{paginate::paginate, post::generate},
    Context, Error,
};
use anime_grubber::agents::waifu_pics::{Categories, NSFW, SFW};

/// Get fancy (or not) characters from waifu.pics
#[poise::command(
    slash_command,
    description_localized("ru", "Аниме персонажи"),
    name_localized("ru", "персонажи"),
    category = "SWF",
    global_cooldown = 3
)]
pub async fn characters(
    ctx: Context<'_>,
    #[description = "Select character"]
    #[description_localized("ru", "Выберите персонажа")]
    #[choices("Waifu", "Neko", "Shinobu", "Megumin")]
    #[name_localized("ru", "тип")]
    #[name_localized("en-US", "type")]
    #[name_localized("ru", "тип")]
    #[name_localized("en-US", "type")]
    _type: &str,
    #[description = "Home many photos need?\nFalse = 1\nTrue = 4"]
    #[description_localized("ru", "Как много фотографий надо?\nFalse = 1\nTrue = 4")]
    #[name_localized("ru", "несколько")]
    many: Option<bool>,
    #[description = "Push it now?"]
    #[description_localized("ru", "Отправить сейчас?")]
    #[name_localized("ru", "сейчас")]
    now: Option<bool>,
) -> Result<(), Error> {
    if now.unwrap_or_default() && !many.unwrap_or_default() {
        ctx.defer().await?;
    } else {
        ctx.defer_ephemeral().await?;
    }
    let sfw_type = match _type {
        "Waifu" => SFW::Waifu,
        "Neko" => SFW::Neko,
        "Shinobu" => SFW::Shinobu,
        "Megumin" => SFW::Megumin,
        _ => return Err("Invalid type".into()),
    };
    let categorie = Categories::SFW(sfw_type);
    let embeds = generate(many.unwrap_or_default(), &categorie, &ctx).await?;
    paginate(ctx, &embeds, categorie, now.unwrap_or_default()).await
}
/// Get fancy (or not) characters with EmOtIoNs from waifu.pics
#[poise::command(
    slash_command,
    description_localized("ru", "Эмоции и действия"),
    name_localized("ru", "эмоции"),
    category = "SWF",
    global_cooldown = 3
)]
pub async fn emotions(
    ctx: Context<'_>,
    #[description = "Select emotion"]
    #[description_localized("ru", "Выберите эмоцию")]
    #[choices("Cry", "Smug", "Blush", "Smile", "Happy", "Wink", "Cringe")]
    #[name_localized("ru", "тип")]
    #[name_localized("en-US", "type")]
    _type: &str,
    #[description = "Home many photos need?\nFalse = 1\nTrue = 4"]
    #[description_localized("ru", "Как много фотографий надо?\nFalse = 1\nTrue = 4")]
    #[name_localized("ru", "несколько")]
    many: Option<bool>,
    #[description = "Push it now?"]
    #[description_localized("ru", "Отправить сейчас?")]
    #[name_localized("ru", "сейчас")]
    now: Option<bool>,
) -> Result<(), Error> {
    if now.unwrap_or_default() && !many.unwrap_or_default() {
        ctx.defer().await?;
    } else {
        ctx.defer_ephemeral().await?;
    }
    let sfw_type = match _type {
        "Cry" => SFW::Cry,
        "Smug" => SFW::Smug,
        "Blush" => SFW::Blush,
        "Smile" => SFW::Smile,
        "Happy" => SFW::Happy,
        "Wink" => SFW::Wink,
        "Cringe" => SFW::Cringe,
        _ => return Err("Invalid type".into()),
    };
    let categorie = Categories::SFW(sfw_type);
    let embeds = generate(many.unwrap_or_default(), &categorie, &ctx).await?;
    paginate(ctx, &embeds, categorie, now.unwrap_or_default()).await
}
/// Get fancy (or not) characters some actions=>> from waifu.pics
#[poise::command(
    slash_command,
    description_localized("ru", "Взаимодействия"),
    name_localized("ru", "действия"),
    category = "SWF",
    global_cooldown = 3
)]
pub async fn actions(
    ctx: Context<'_>,
    #[description = "Select action"]
    #[description_localized("ru", "Выберите действие")]
    #[choices(
        "Bully", "Cuddle", "Hug", "Kiss", "Lick", "Pat", "Bonk", "Yeet", "Highfive", "Handhold",
        "Nom", "Bite", "Glomp", "Slap", "Kill", "Kick", "Poke", "Dance"
    )]
    #[name_localized("ru", "тип")]
    #[name_localized("en-US", "type")]
    _type: &str,
    #[description = "Home many photos need?\nFalse = 1\nTrue = 4"]
    #[description_localized("ru", "Как много фотографий надо?\nFalse = 1\nTrue = 4")]
    #[name_localized("ru", "несколько")]
    many: Option<bool>,
    #[description = "Push it now?"]
    #[description_localized("ru", "Отправить сейчас?")]
    #[name_localized("ru", "сейчас")]
    now: Option<bool>,
) -> Result<(), Error> {
    if now.unwrap_or_default() && !many.unwrap_or_default() {
        ctx.defer().await?;
    } else {
        ctx.defer_ephemeral().await?;
    }
    let sfw_type = match _type {
        "Bully" => SFW::Bully,
        "Cuddle" => SFW::Cuddle,
        "Hug" => SFW::Hug,
        "Kiss" => SFW::Kiss,
        "Lick" => SFW::Lick,
        "Pat" => SFW::Pat,
        "Bonk" => SFW::Bonk,
        "Yeet" => SFW::Yeet,
        "Highfive" => SFW::Highfive,
        "Handhold" => SFW::Handhold,
        "Nom" => SFW::Nom,
        "Bite" => SFW::Bite,
        "Glomp" => SFW::Glomp,
        "Slap" => SFW::Slap,
        "Kill" => SFW::Kill,
        "Kick" => SFW::Kick,
        "Poke" => SFW::Poke,
        "Dance" => SFW::Dance,
        _ => return Err("Invalid type".into()),
    };
    let categorie = Categories::SFW(sfw_type);
    let embeds = generate(many.unwrap_or_default(), &categorie, &ctx).await?;
    paginate(ctx, &embeds, categorie, now.unwrap_or_default()).await
}

/// Get fancy (or not) characters with nude style from waifu.pics
#[poise::command(
    slash_command,
    ephemeral,
    description_localized("ru", "кошка жена и миска риса (18+)"),
    name_localized("ru", "вайфу_порно"),
    nsfw_only,
    global_cooldown = 3,
    category = "NSFW",
    guild_only
)]
pub async fn waifu_nsfw(
    ctx: Context<'_>,
    #[description = "Select type"]
    #[description_localized("ru", "Выберите тип")]
    #[choices("Waifu", "Neko", "Trap", "Blowjob")]
    #[name_localized("ru", "тип")]
    #[name_localized("en-US", "type")]
    _type: &str,
    #[description_localized("ru", "Как много фотографий надо?\nFalse = 1\nTrue = 4")]
    #[name_localized("ru", "несколько")]
    many: Option<bool>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let nsfw_type = match _type {
        "Waifu" => NSFW::Waifu,
        "Neko" => NSFW::Neko,
        "Trap" => NSFW::Trap,
        "Blowjob" => NSFW::Blowjob,
        _ => return Err("Invalid type".into()),
    };
    let categorie = Categories::NSFW(nsfw_type);
    let embeds = generate(many.unwrap_or_default(), &categorie, &ctx).await?;
    paginate(ctx, &embeds, categorie, true).await
}
