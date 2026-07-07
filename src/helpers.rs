pub use poise::serenity_prelude as serenity;

#[macro_export]
macro_rules! create_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction.create_response(
            $ctx,
            $crate::helpers::serenity::CreateInteractionResponse::Message($message)
        ).await?
    };
}

#[macro_export]
macro_rules! create_edit_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction.create_response(
            $ctx,
            $crate::helpers::serenity::CreateInteractionResponse::UpdateMessage($message)
        ).await?
    };
}