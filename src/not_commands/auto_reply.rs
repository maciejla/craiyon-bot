use crate::utils::{Context, DisplayUser};

pub async fn auto_reply(ctx: Context) {
    let text = match &ctx.message.text {
        Some(text) => text.to_lowercase(),
        None => return,
    };

    let words = text.split_ascii_whitespace().collect::<Vec<_>>();

    let reply = if words.contains(&"tomasz") && words.contains(&"fryta") {
        "real madryt"
    } else {
        return;
    };

    if ctx
        .ratelimiter
        .write()
        .unwrap()
        .update_rate_limit((ctx.user.id, "auto_reply".to_string()), ctx.message.date)
        .is_some()
    {
        log::warn!("Auto reply rate limit exceeded by {} ({reply:?})", ctx.user.format_name());
        return;
    }

    ctx.reply(reply).await.ok();
}
