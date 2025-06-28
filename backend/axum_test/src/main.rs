use axum::{routing::{get}, Router, Json};

/*
    -router ile json formatında ki girdileri belirliyoruz burada ki / anasayfayı işaret ediyor
    -axum ile gelen istekleri dinliyoruz.Test için postman kullanabilirsin veya browser'dan http://localhost:8080/vehicle aratarak istek gönderebilirsin.

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

#[derive(serde::Serialize, serde::Deserialize)]
struct Vehicle{
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>
}

// Browser'dan veya Postman'den GET isteği gönderildiğinde çalışır.
#[axum::debug_handler] // hata ayıklama için
async fn vehicle_get() -> axum::Json<Vehicle> {
    Json::from(
        Vehicle {
            manufacturer: "BMW".to_string(),
            model: "3".to_string(),
            year: 2025,
            id: Some(uuid::Uuid::new_v4().to_string()),
        }
    )
}

async fn vehicle_post(Json(vehicle): Json<Vehicle>) -> Json<Vehicle> {
    // Gelen veriyi işleyebiliriz, burada sadece geri döndürüyoruz
    // Örneğin, veritabanına kaydedebiliriz veya başka işlemler yapabiliriz.
    Json(vehicle)

}