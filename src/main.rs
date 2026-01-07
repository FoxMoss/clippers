use crate::ffi::{init, end, embed_text, embed_image};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cliprust/clip.h");
        include!("cliprust/rust_interface.h");

        fn init(path: String);
        fn embed_text(text: String) -> Vec<f32>;
        fn embed_image(path: String) -> Vec<f32>;
        fn end();
    }
}

pub fn rust_init(model_path: &str) {
    init(model_path.to_string());
}

pub fn rust_end() {
    end();
}

pub fn rust_embed_text(text: String) -> Option<Vec<f32>>{
    let vec = embed_text(text);

    if vec.len() == 0{
        return None;
    }
    return Some(vec);
}

pub fn rust_embed_image(path: String) -> Option<Vec<f32>>{
    let vec = embed_image(path);

    if vec.len() == 0{
        return None;
    }
    return Some(vec);
}


// fn main(){
//     rust_init();
//
//     let text_embbedding = rust_embed_text("a tall man".to_string());
//     let image_embbedding = rust_embed_image("/home/foxmoss/Downloads/tallman.jpg".to_string());
//     print!("{:?}", text_embbedding);
//     print!("{:?}", image_embbedding);
//
//     rust_end();
// }
