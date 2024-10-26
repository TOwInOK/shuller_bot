use poise::samples::HelpConfiguration;

use crate::{Context, Error};
/// Helpful information
#[poise::command(slash_command, category = "Utility")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[description_localized("ru", "Комманда чтоб понять что к чему")]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!(
                "$This: {} <<<- AbouT ->>> {}",
                ctx.invoked_command_name(),
                c
            )),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom: "
        - `now`
        Use `now` if you want to send in chat instead of selecting an image.
        This functionality is only needed for user mode and is useless for guild mode.
        Also `now` only works with 1 image with instant send.
        ",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
