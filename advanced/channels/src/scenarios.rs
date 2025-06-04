use std::sync::mpsc::{channel, Sender, Receiver}; // Standart MPSC kanalları
use std::thread; // Thread işlemleri için
use std::time::Duration; // Süre tanımlama için
use tokio::sync::mpsc; // Tokio MPSC kanalları için

// process_reports: Birden çok thread ile dosya işleme ve MPSC ile durum raporlama
// Ne işe yarar? Dosya işleme sonuçlarını kanal üzerinden toplar
// Neden kullanılır? Thread'ler arası güvenli ve sıralı veri aktarımı için
pub fn process_reports() {
    // channel(): Standart MPSC kanalı oluşturur
    // Teknik incelik: tx klonlanabilir, rx tektir (multi-producer, single-consumer)
    let (transmitter, receiver): (Sender<String>, Receiver<String>) = channel();

    let reports = [
        "salary.json",
        "invoices.json",
        "summary.json",
        "personnel.json",
    ];

    // Her rapor için bir thread başlatılır
    // Mantık: Paralel dosya işleme simülasyonu
    for report in reports {
        let transmitter = transmitter.clone(); // Yeni gönderici klonu
        thread::spawn(move || {
            let sleep_time = 3 ;
            // İşleme başlangıç bildirimi
            transmitter
                .send(format!("Processing '{}' report...", report))
                .expect("Failed to send processing message");
            
            // Dosya işleme simülasyonu
            // Teknik incelik: thread::sleep, gerçek işleme yerine geçer
            thread::sleep(Duration::from_secs(sleep_time));

            // İşleme tamamlanma bildirimi
            transmitter
                .send(format!("Finished processing '{}' in {} seconds", report, sleep_time))
                .expect("Failed to send completion message");
        });
    }

    // transmitter düşürülür, kanal kapanır
    // Teknik incelik: Tüm tx'ler drop edilince rx.iter() biter
    drop(transmitter);
    println!("Started the processing reports");

    // receiver.iter(): Kanal kapanana kadar mesajları toplar
    // Mantık: Tüm thread'lerden gelen durumları sıralı yazdırır
    for result in receiver {
        println!("Status: {}", result);
    }
    println!("Completed the processing reports");
}

// do_with_standard: Standart MPSC ile senkron mesajlaşma örneği
// Ne işe yarar? Thread'lerden mesaj toplar, ancak ana thread'i bloklar
// Neden kullanılır? Basit senaryolarda düşük ek yükle çalışır
pub fn do_with_standard() {
    // Standart MPSC kanalı oluşturur
    let (transmitter, receiver): (Sender<String>, Receiver<String>) = channel();

    // 5 thread başlatılır, her biri mesaj gönderir
    // Teknik incelik: tx.clone(), birden çok gönderici sağlar
    for i in 1..=5 {
        let tx_clone = transmitter.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5)); // Sabit gecikme
            tx_clone
                .send(format!("Task {} completed", i))
                .expect("Failed to send task completion");
        });
    }

    // transmitter düşürülür, kanal kapanır
    drop(transmitter);
    println!("Waiting for all tasks...");

    // Ana thread'de ek iş simülasyonu
    // Teknik incelik: receiver.recv() blokladığı için bu döngü sırayla çalışır
    for i in 0..10 {
        thread::sleep(Duration::from_secs(1));
        println!("Main task is working...Counting {}", i);
    }

    // receiver.recv(): Mesaj gelene kadar bloklar
    // Mantık: Mesajları sırayla alır, asenkron değil
    while let Ok(message) = receiver.recv() {
        println!("{}", message);
    }
    println!("All tasks completed!");
}

// do_with_tokio: Tokio MPSC ile asenkron mesajlaşma örneği
// Ne işe yarar? Ana thread'i bloklamadan mesaj toplar
// Neden kullanılır? Gerçek asenkronluk için, özellikle yüksek eşzamanlılıkta
pub async fn do_with_tokio() {
    // Tokio MPSC kanalı, 10 mesaj kapasiteli
    // Teknik incelik: Kapasite sınırı, tamponlama sağlar
    let (transmitter, mut receiver) = mpsc::channel(10);

    // 5 asenkron görev başlatılır
    // Mantık: tokio::spawn, Tokio runtime'da çalışır
    for i in 1..=5 {
        let tx_clone = transmitter.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await; // Asenkron bekleme
            tx_clone
                .send(format!("Task {} completed", i))
                .await
                .expect("Failed to send task completion");
        });
    }

    // transmitter düşürülür, kanal kapanır
    drop(transmitter);
    println!("Waiting for all tasks...");

    // Asenkron döngü, ana thread'i bloklamaz
    // Teknik incelik: tokio::spawn, döngüyü ayrı bir görevde çalıştırır
    tokio::spawn(async {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Main task is working...Counting {}", i);
        }
    });

    // receiver.recv(): Asenkron mesaj alma
    // Mantık: Mesajlar geldikçe işlenir, ana thread serbest kalır
    while let Some(message) = receiver.recv().await {
        println!("{}", message);
    }
    println!("All tasks completed!");
}