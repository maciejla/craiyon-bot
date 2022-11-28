use std::sync::Arc;

use async_trait::async_trait;

use super::CommandError::MissingArgument;
use super::{CommandResult, CommandTrait};
use crate::apis::google;
use crate::command_context::CommandContext;

#[derive(Default)]
pub struct Autocomplete;

#[async_trait]
impl CommandTrait for Autocomplete {
    fn command_names(&self) -> &[&str] {
        &["autocomplete", "complete", "google"]
    }

    fn description(&self) -> Option<&'static str> {
        Some("autocompletes a query with Google")
    }

    async fn execute(&self, ctx: Arc<CommandContext>, arguments: Option<String>) -> CommandResult {
        let query = arguments.ok_or(MissingArgument("text to autocomplete"))?;

        let completions = google::complete(ctx.http_client.clone(), &query).await?;
        let query_lowercase = query.to_lowercase();
        ctx.reply_html(
            completions.into_iter().find(|c| *c != query_lowercase).ok_or("no autocompletions")?,
        )
        .await?;

        Ok(())
    }
}
