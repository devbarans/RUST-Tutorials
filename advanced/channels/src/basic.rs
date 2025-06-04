use std::sync::mpsc::{channel, Sender, Receiver}; // MPSC kanalları için gerekli modül
use std::thread; // Thread işlemleri için
use std::time::Duration; // Thread'lerin beklemesi için süre tanımlamak

// hello_channels: MPSC kanallarıyla thread'ler arası iletişim örneği
// Ne işe yarar? Thread'ler arasında güvenli veri aktarımı sağlar
// Neden kullanılır? Rust'ta thread'ler arası veri paylaşımı için güvenli bir mekanizma
pub fn hello_channels() {
    // channel(): Bir MPSC kanalı oluşturur
    // - tx (transmitter): Veriyi gönderen taraf
    // - rx (receiver): Veriyi alan taraf
    // Teknik incelik: MPSC, birden çok üretici (tx) ve tek tüketici (rx) destekler
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // Örnek mesaj
    let message = String::from("Hello Channels!");

    // Yeni bir thread oluşturuyoruz
    // thread::spawn: Yeni bir iş parçacığı başlatır
    // move: message ve tx'in sahipliğini closure'a taşır
    let handle = thread::spawn(move || {
        // Teknik incelik: move, değişkenlerin closure'a sahip olmasını sağlar
        // tx.send: Mesajı kanala gönderir
        // unwrap: Hata yönetimi için basit bir yöntem, ama üretim kodunda dikkatli kullanılmalı
        tx.send(message).expect("Failed to send message");
        // Ekstra mesaj gönderimi: Birden fazla mesaj örneği
        tx.send(String::from("Second message")).expect("Failed to send second message");
    });

    // Ana thread'de mesajları alıyoruz
    // rx.recv: Mesaj gelene kadar bloklar
    // Teknik incelik: recv, Result döner; hata durumunda kanal kapanmışsa Err döner
    match rx.recv() {
        Ok(received) => println!("Received message: {}", received),
        Err(e) => eprintln!("Receive error: {}", e),
    }

    // İkinci mesajı alma
    // Teknik incelik: Kanaldaki tüm mesajları sırayla alır
    match rx.recv() {
        Ok(received) => println!("Received second message: {}", received),
        Err(e) => eprintln!("Second receive error: {}", e),
    }

    // Thread'in tamamlanmasını bekler
    // Neden kullanılır? Ana thread'in, çocuk thread tamamlanmadan kapanmasını önler
    handle.join().expect("Failed to join thread");

    // Bonus: Kanalın hala açık olduğunu test etmek
    // rx.try_recv: Bloklamadan mesaj almayı dener
    match rx.try_recv() {
        Ok(unexpected) => println!("Unexpected message: {}", unexpected),
        Err(e) => println!("Channel status: {}", e), // Kanal boşsa veya kapalıysa
    }
}

// Örnek: Birden fazla thread'den mesaj gönderme
// Ne işe yarar? MPSC'nin "multi-producer" özelliğini gösterir
pub fn multi_producer_example() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // Birden fazla transmitter klonlama
    // Teknik incelik: tx.clone(), yeni bir gönderici oluşturur
    let tx2 = tx.clone();

    // İlk thread: Bir mesaj gönderir
    thread::spawn(move || {
        tx.send(String::from("Message from thread 1")).expect("Failed to send from thread 1");
        // Simüle edilmiş bir gecikme
        thread::sleep(Duration::from_millis(100));
        tx.send(String::from("Additional message from thread 1")).expect("Failed to send additional from thread 1");
    });

    // İkinci thread: Başka bir mesaj gönderir
    thread::spawn(move || {
        tx2.send(String::from("Message from thread 2")).expect("Failed to send from thread 2");
    });

    // Ana thread: Mesajları toplamak için döngü
    // Teknik incelik: rx.into_iter() ile kanaldaki tüm mesajlar alınır
    for received in rx {
        println!("Received: {}", received);
    }
}


pub fn multi_producer_2(){
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // Birden fazla thread oluşturma
    // Teknik incelik: Her thread kendi tx klonunu kullanır
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let message = format!("Message from thread {}", i);
            tx_clone.send(message).expect("Failed to send message");
            thread::sleep(Duration::from_millis(50)); // Simüle edilmiş gecikme
        });
    }
    
    drop(tx); // Tüm tx'leri düşür, böylece rx kapanır

    // Mesajları alıyoruz
    // Teknik incelik: rx, kapanana kadar mesajları alır
    for received in rx {
        println!("Received: {}", received);
    }

    println!("End of multi_producer_2 example");

  
}