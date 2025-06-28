use axum::{routing::{get, post}, Router};

/*
    -router ile json formatında ki girdileri belirliyoruz burada ki / anasayfayı işaret ediyor


*/

#[tokio::main]
async fn main() {
    // Router oluşturduk
    let vehicle_router = Router::new()
    .route("/vehicle", 
    get(vehicle_get).
    post(vehicle_post));

    // ip ve port listener(tcp) tanımlıyoruz
    let addr = "0.0.0.0:8080";  // Daha yaygın kullanılan bir port
    let listener = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap();
 
    // axum ile web sunucusunu başlatıyoruz
    axum::serve(listener, vehicle_router)
    .await
    .unwrap();

}

async fn vehicle_get(){

}

async fn vehicle_post(){

}