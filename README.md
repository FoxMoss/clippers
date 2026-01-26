# Cliprs

[clip.cpp](https://github.com/monatis/clip.cpp) bindings to rust.

## Grabing a model file

I've been using
[huggingface.co/mys/ggml_clip-vit-large-patch14](https://huggingface.co/mys/ggml_clip-vit-large-patch14)
two tower models for testing. All quantization levels have worked great!

## Install

To add to your project:

```
cargo add cliprs
```

## Usage

For a detailed example, see [`src/main.rs`](./src/main.rs).

But basic usage looks like
```rust
use cliprs::ClipModel;

fn main() {
    // Loading model
    let model = ClipModel::new("/path/to/model.gguf");

    // Embedding text
    let text_embedding = model.embed_text("text").unwrap();

    // Embedding an image
    let image_embedding = model.embed_image("/path/to/image.jpg").unwrap();

    // Comparing image and text. You can also compare embeddings of the same type.
    let similarity = model.embed_compare(&text_embedding, &image_embedding);
    println!("Similarity: {}", similarity);

    // Prints all warnings during processing
    for warning in cliprs::poll_warnings() {
        eprintln!("Warning: {}", warning);
    }
}
```
