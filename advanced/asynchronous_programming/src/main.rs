mod api_sample;

/// Ana fonksiyon, Tokio çalışma zamanını başlatır
#[tokio::main]
async fn main() {
    println!("Starting API sample...");
    api_sample::run().await;
    println!("API sample completed.");
}
