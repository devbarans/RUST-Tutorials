use tokio::task; // Asenkron görevler oluşturmak için Tokio'nun task modülü
use reqwest; // HTTP istekleri için reqwest kütüphanesi

/// Belirtilen URL'den veri çeken asenkron fonksiyon
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    // URL'ye HTTP GET isteği gönder, yanıtı asenkron olarak bekle
    let response = reqwest::get(url).await?;
    // Yanıt gövdesini String olarak al, asenkron olarak
    response.text().await
}

/// İki URL'den paralel olarak veri çeken asenkron fonksiyon
pub async fn run() {
    // İki asenkron görevi paralel çalıştırmak için task::spawn kullanılır
    // task::spawn, görevleri Tokio'nun iş parçacığı havuzunda yönetir
    let task_a = task::spawn(async {
        // İlk URL'den veri çek, sonucu Result<String, reqwest::Error> olarak döndür
        fetch_data("https://jsonplaceholder.typicode.com/posts/1").await
    });

    // İkinci görev, ikinci URL'den veri çeker
    let task_b = task::spawn(async {
        // İkinci URL'den veri çek, sonucu Result<String, reqwest::Error> olarak döndür
        fetch_data("https://jsonplaceholder.typicode.com/posts/2").await
    });

    // Her iki görevin tamamlanmasını paralel olarak bekle
    // tokio::join!, görev sonuçlarını bir tuple olarak döner
    let (result_a, result_b) = tokio::join!(task_a, task_b);

    // Görev sonuçlarını eşleştir ve işle
    match (result_a, result_b) {
        // Her iki görev başarılıysa sonuçları yazdır
        (Ok(data_a), Ok(data_b)) => {
            println!("Data A: {:?}", data_a);
            println!("Data B: {:?}", data_b);
        }
        // Herhangi bir görevde hata varsa hata mesajını yazdır
        (Err(e), _) | (_, Err(e)) => {
            // Hata mesajını standart hata akışına (stderr) yaz
            eprintln!("Veri çekme hatası: {}", e);
        }
    }
}
