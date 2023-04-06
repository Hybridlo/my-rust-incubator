use clap::Parser;
use tokio::io::AsyncWriteExt;
use std::{thread::{available_parallelism, self}, path::PathBuf};

const OUTPUT_FOLDER: &str = "download/";

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = available_parallelism().unwrap().get())]
    max_threads: usize,
    file: PathBuf
}

async fn download_files(links: &[String]) {
    let requests = links.into_iter().map(|link| async move {
        if let Ok(response) = reqwest::get(link).await {
            if let Ok(mut bytes) = response.bytes().await {
                let link_last_part = link.split("/").last().expect("To have parts in the link");

                let mut new_file = tokio::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(OUTPUT_FOLDER.to_string() + link_last_part + ".html")
                    .await
                    .expect("To be able to open a file");

                new_file.write_all_buf(&mut bytes).await.expect("To be able to write the buffer");
            }
        }
    }).collect::<Vec<_>>();

    futures::future::join_all(requests).await;
}

fn main() {
    let args = Args::parse();

    let links = std::fs::read_to_string(args.file).expect("Being able to read the links file")
        .lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();

    // create the path if it doesn't exist
    std::fs::create_dir_all(OUTPUT_FOLDER).expect("To be able to create output folder");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.max_threads)
        .enable_all()
        .build().expect("to create a Tokio Runtime");

    runtime.block_on(async move {
        futures::future::join_all(
            links
                .chunks((links.len() as f64 / args.max_threads as f64).ceil() as usize)
                .map(|links_chunk| {
                    download_files(links_chunk)
                })
                .collect::<Vec<_>>()
        ).await;
    });
}
