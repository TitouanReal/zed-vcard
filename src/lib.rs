use zed_extension_api as zed;

struct VCardExtension {}

impl zed::Extension for VCardExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "./vcard-lsp".to_owned(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(VCardExtension);
