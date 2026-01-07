#pragma once

#include "rust/cxx.h"
#include <optional>
#include <string>
#include <vector>

void init(rust::String model_path);
rust::vec<float> embed_text(rust::String text);
rust::vec<float> embed_image(rust::String path);
void end();
