use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .include_file(out_dir.join("_includes.rs"))
        .file_descriptor_set_path(out_dir.join("rest_articles_axum_features_descriptor.bin"))
        .compile(
            &[
                "proto/api/articles/article/create_update_articles_v1.proto",
                "proto/api/articles/article/delete_articles_v1.proto",
                "proto/api/articles/article/get_article_list_v1.proto",
                "proto/api/articles/article/get_article_v1.proto",
                "proto/core/create_update_entities_response.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
