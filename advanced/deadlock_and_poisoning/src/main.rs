use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Thread'ler arası veri paylaşımı ve kilitlenme senaryolarını gösterir
fn main() {
    println!("=== Poisoning Örneği ===");
    poisoning();
    println!("\n=== Deadlock Örneği ===");
    deadlock();
}

// poisoning: Mutex zehirlenmesini ve kurtarmayı gösterir
// Ne işe yarar? Bir thread panik atarsa Mutex zehirlenir, kurtarma stratejisi uygulanır
// Teknik incelik: unwrap_or_else ile zehirlenmiş kilitten veri alınır
pub fn poisoning() {
    // Ortak dosya: Arc ile paylaşılır, Mutex ile kilitlenir
    // Neden Arc? Birden çok thread dosyayı paylaşsın
    // Neden Mutex? Aynı anda sadece bir thread yazsın
    let log_file = Arc::new(Mutex::new(File::create("log.txt").expect("Failed to create file")));

    // Arc klonu: Thread’e referans verir
    // Teknik incelik: Arc, referans sayımıyla paylaşımı güvenli yapar
    let log_file_clone = Arc::clone(&log_file);

    // İlk thread: Panik atarak zehirlenme simülasyonu
    let handle = thread::spawn(move || {
        let mut file = log_file_clone.lock().expect("Failed to lock mutex");
        writeln!(file, "Thread started").expect("Failed to write");
        panic!("Simulated panic in thread"); // Mutex zehirlenir
    });

    // İkinci thread: Zehirlenmiş Mutex’ten kurtarma
    // Mantık: unwrap_or_else ile zehirlenmiş kilitten veri alınır
    let log_file_clone = Arc::clone(&log_file);
    let recovery_thread = thread::spawn(move || {
        let mut file = log_file_clone
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()); // Zehirlenmeyi tolere et
        thread::sleep(Duration::from_secs(3)); // İş simülasyonu
        writeln!(file, "Recovery thread: Handling panic").expect("Failed to write");
    });

    // Thread’lerin tamamlanmasını bekle
    // Teknik incelik: join, ana thread’in erken kapanmasını önler
    recovery_thread.join().expect("Failed to join recovery thread");
    handle.join().ok(); // Panik olduğu için Ok değil, hata yoksayılır
    println!("Log file operations completed");
}

// deadlock: Thread’ler arası kilitlenme önlenmiş transfer örneği
// Ne işe yarar? Kilit alma sırasını sabitleyerek deadlock’u engeller
// Teknik incelik: Her zaman aynı kilit sırası (my_account → other_account) kullanılır
pub fn deadlock() {
    // İki hesap: Arc ve Mutex ile paylaşılır
    // Mantık: Thread’ler hesaplar arasında para transferi yapar
    let my_account = Arc::new(Mutex::new(Account {
        owner: String::from("Alice"),
        balance: 100.0,
    }));
    let other_account = Arc::new(Mutex::new(Account {
        owner: String::from("Bob"),
        balance: 50.0,
    }));

    // Thread 1: Alice’ten Bob’a transfer
    // Kilit sırası: my_account → other_account
    let my_account_clone = Arc::clone(&my_account);
    let other_account_clone = Arc::clone(&other_account);
    let handle = thread::spawn(move || {
        let mut my_account_lock = my_account_clone.lock().expect("Failed to lock my_account");
        // İş simülasyonu
        thread::sleep(Duration::from_millis(100));
        let mut other_account_lock = other_account_clone.lock().expect("Failed to lock other_account");

        my_account_lock.balance -= 10.0;
        other_account_lock.balance += 10.0;
        println!("{} transferred 10 to {}", my_account_lock.owner, other_account_lock.owner);
    });

    // Thread 2: Bob’dan Alice’e transfer
    // Kilit sırası: my_account → other_account (Aynı sıra, deadlock önlenir)
    let my_account_clone = Arc::clone(&my_account);
    let other_account_clone = Arc::clone(&other_account);
    let handle_2 = thread::spawn(move || {
        let mut my_account_lock = my_account_clone.lock().expect("Failed to lock my_account");
        // İş simülasyonu
        thread::sleep(Duration::from_millis(100));
        let mut other_account_lock = other_account_clone.lock().expect("Failed to lock other_account");

        other_account_lock.balance -= 20.0;
        my_account_lock.balance += 20.0;
        println!("{} transferred 20 to {}", other_account_lock.owner, my_account_lock.owner);
    });

    // Thread’lerin bitmesini bekle
    // Mantık: Kilit sırası sabit olduğu için deadlock olmaz
    handle.join().expect("Failed to join thread 1");
    handle_2.join().expect("Failed to join thread 2");
    println!("Deadlock example completed");
}

//Thread’ler arasında paylaşılan veri
#[derive(Debug)]
struct Account {
    owner: String,
    balance: f64,
}