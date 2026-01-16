use std::sync::Mutex;

static WARNINGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn log_warning(message: String);
    }

    unsafe extern "C++" {
        include!("cliprs/clip.h");
        include!("cliprs/rust_interface.h");

        type clip_ctx;

        fn init(path: String) -> *mut clip_ctx;
        unsafe fn embed_text(ctx: *const clip_ctx, text: String) -> Result<Vec<f32>>;
        unsafe fn embed_image(ctx: *const clip_ctx, path: String) -> Result<Vec<f32>>;
        unsafe fn embed_compare(ctx: *const clip_ctx, p1: &Vec<f32>, p2: &Vec<f32>) -> f32;
        unsafe fn end(ctx: *mut clip_ctx);
    }
}

pub fn log_warning(message: String) {
    if let Ok(mut warnings) = WARNINGS.lock() {
        warnings.push(message);
    }
}

pub struct ClipModel {
    ctx: *mut ffi::clip_ctx,
}

// Required for Send/Sync since we're using raw pointers
unsafe impl Send for ClipModel {}
unsafe impl Sync for ClipModel {}

impl ClipModel {
    /// Initializes ClipModel with a model path.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// model = ClipModel::new("path/to/model.gguf")
    /// ```
    pub fn new(model_path: impl Into<String>) -> Self {
        Self {
            ctx: ffi::init(model_path.into()),
        }
    }

    /// Allows you to compare two embeddings.
    pub fn embed_compare(&self, p1: &Vec<f32>, p2: &Vec<f32>) -> f32 {
        unsafe { ffi::embed_compare(self.ctx, p1, p2) }
    }

    /// Embeds text.
    pub fn embed_text(&self, text: impl Into<String>) -> Result<Vec<f32>, String> {
        match unsafe { ffi::embed_text(self.ctx, text.into()) } {
            Ok(embed) if embed.is_empty() => Err("Text embedding is empty".to_string()),
            Ok(embed) => Ok(embed),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Embeds an image from a path.
    pub fn embed_image(&self, path: impl Into<String>) -> Result<Vec<f32>, String> {
        let path: String = path.into();
        match unsafe { ffi::embed_image(self.ctx, path.clone()) } {
            Ok(embed) if embed.is_empty() => Err(format!("Embedding for {} is empty", path)),
            Ok(embed) => Ok(embed),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Returns all new warning messages since the last call.
    pub fn poll_warnings(&self) -> Vec<String> {
        WARNINGS
            .lock()
            .map(|mut w| w.drain(..).collect())
            .unwrap_or_default()
    }
}

impl Drop for ClipModel {
    fn drop(&mut self) {
        unsafe {
            ffi::end(self.ctx);
        }
    }
}