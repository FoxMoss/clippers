use std::{env::args, io::{Write, stdin, stdout}};

use cliprs::ClipModel;

fn main() {
    // Loading model
    let model = ClipModel::new(
        args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("Please provide a model path as the first argument"),
    );

    // Getting image path
    print!("Image path: ");
    stdout().flush().expect("Failed to flush stdout");
    let mut image_path = String::new();
    stdin()
        .read_line(&mut image_path)
        .expect("Failed to read input");

    // Embedding image
    let image_embedding: Vec<f32> = model
        .embed_image(image_path.trim())
        .expect("Failed to embed image");

    // Getting text
    print!("Input text: ");
    stdout().flush().expect("Failed to flush stdout");
    let mut text = String::new();
    stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    // Embedding text
    let text_embedding: Vec<f32> = model.embed_text(text.trim()).expect("Failed to embed text");

    // Comparing embeddings
    let similarity = model.embed_compare(&text_embedding, &image_embedding);
    println!("Similarity: {}", similarity);

    // Any warnings are logged and can be retrieved using poll_warnings()
    for warning in cliprs::poll_warnings() {
        eprintln!("Warning: {}", warning);
    }
}
