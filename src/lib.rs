use zed_extension_api as zed;

struct VCardExtension {}

impl zed::Extension for VCardExtension {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(VCardExtension);
