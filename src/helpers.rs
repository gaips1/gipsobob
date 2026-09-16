use super::types::*;

use serenity::{CreateInteractionResponse, CreateInteractionResponseMessage};

pub trait InteractionExt {
    async fn reply(
        &self,
        ctx: &serenity::Context,
        msg: serenity::CreateInteractionResponseMessage,
    ) -> serenity::Result<()>;
    async fn edit_reply(
        &self,
        ctx: &serenity::Context,
        msg: serenity::CreateInteractionResponseMessage,
    ) -> serenity::Result<()>;
}

macro_rules! impl_interaction_ext {
    ($( $t:ty ),* $(,)?) => {
        $(
            impl InteractionExt for $t {
                async fn reply(&self, ctx: &serenity::Context, msg: CreateInteractionResponseMessage) -> serenity::Result<()> {
                    self.create_response(ctx, CreateInteractionResponse::Message(msg)).await
                }

                async fn edit_reply(&self, ctx: &serenity::Context, msg: CreateInteractionResponseMessage) -> serenity::Result<()> {
                    self.create_response(ctx, CreateInteractionResponse::UpdateMessage(msg)).await
                }
            }
        )*
    };
}

impl_interaction_ext!(
    serenity::CommandInteraction,
    serenity::ComponentInteraction,
    serenity::ModalInteraction,
);

pub fn resolve_data_path(relative: &str) -> std::path::PathBuf {
    if cfg!(debug_assertions) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join(relative)
    } else {
        let exe_dir = std::env::current_exe()
            .expect("failed to get current exe path")
            .parent()
            .unwrap()
            .join("bot_data")
            .to_path_buf();

        let file_name = std::path::Path::new(relative)
            .file_name()
            .expect("relative path has no file name");

        exe_dir.join(file_name)
    }
}
