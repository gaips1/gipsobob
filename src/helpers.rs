pub use poise::serenity_prelude as serenity;

#[macro_export]
macro_rules! create_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction
            .create_response(
                $ctx,
                $crate::helpers::serenity::CreateInteractionResponse::Message($message),
            )
            .await?
    };
}

#[macro_export]
macro_rules! create_edit_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction
            .create_response(
                $ctx,
                $crate::helpers::serenity::CreateInteractionResponse::UpdateMessage($message),
            )
            .await?
    };
}

pub fn resolve_data_path(relative: &str) -> std::path::PathBuf {
    if cfg!(debug_assertions) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join(relative)
    } else {
        let exe_dir = std::env::current_exe()
            .expect("failed to get current exe path")
            .parent()
            .unwrap()
            .to_path_buf();

        let file_name = std::path::Path::new(relative)
            .file_name()
            .expect("relative path has no file name");

        exe_dir.join(file_name)
    }
}
