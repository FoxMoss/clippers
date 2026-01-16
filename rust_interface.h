#pragma once

#include "rust/cxx.h"
#include <optional>
#include <string>
#include <vector>

// Forward declare from clip.h
extern "C" {
    struct clip_ctx;
    struct clip_image_u8;
    struct clip_image_f32;
    struct clip_tokens;
}

// FFI function declarations
struct clip_ctx * init(rust::String model_path);
rust::vec<float> embed_text(const struct clip_ctx * ctx, rust::String text);
rust::vec<float> embed_image(const struct clip_ctx * ctx, rust::String path);
float embed_compare(const struct clip_ctx * ctx, const rust::vec<float> & p1, const rust::vec<float> & p2);
void end(struct clip_ctx * ctx);

extern "C" void clip_log_warning(const char* message);