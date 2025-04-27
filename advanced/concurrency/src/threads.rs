use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// THREAD'LER NEDİR?
// - Thread'ler, bir programda eşzamanlı görevlerin yürütülmesini sağlar.
// - Rust’ın `std::thread` modülü, thread oluşturma ve yönetme araçları sunar.
// - Thread’ler bağımsız çalışır ve çok çekirdekli sistemlerde paralel işlem yapabilir.
// - Rust’ın sahiplik (ownership) ve ödünç alma (borrowing) sistemi, veri yarışlarını
//   (data races) derleme zamanında önleyerek thread güvenliğini garanti eder.

// THREAD’LER NEDEN KULLANILIR?
// - Paralellik: Çoklu CPU çekirdeklerini kullanarak performansı artırma.
// - Eşzamanlılık: Birden fazla görevi aynı anda yürütme (ör. kullanıcı arayüzü
//   yanıtliligini korurken veri işleme).
// - Ana Thread’i Serbest Bırakma: Uzun süren görevleri ana thread’den ayırarak
//   programın yanıt verebilirliğini koruma.

// `move` ANAHTAR KELİMESİ NEDİR?
// - `move`, bir kapanışın (closure) değişkenlerin sahipliğini almasını sağlar.
// - Kullanım:
//   - `move` olmadan: Kapanış, değişkenleri ödünç alır. Eğer ana kapsam değişkenleri
//     düşürürse (drop), thread hala bu değişkenlere erişmeye çalışırsa derleme hatası olur.
//   - `move` ile: Kapanış, değişkenlerin sahipliğini alır ve thread kendi kendine yeterli
//     hale gelir. Bu, thread’in ana kapsamdan daha uzun yaşayacağı durumlarda gereklidir.
// - Dezavantaj: `move` kullanıldığında, değişkenler ana kapsamda kullanılamaz hale gelir.

// `Arc` (Atomic Reference Counting) NEDİR?
// - `Arc`, thread-safe bir referans sayacıdır. Bir verinin birden fazla thread tarafından
//   paylaşılmasını sağlar.
// - Nasıl Çalışır?
//   - `Arc`, bir veriyi sarar ve her `Arc::clone` çağrısı referans sayısını artırır (atomik olarak).
//   - Bir `Arc` kopyası düştüğünde, referans sayısı azalır. Son kopya düştüğünde veri serbest bırakılır.
// - Neden Kullanılır?
//   - Thread’ler arasında veri paylaşımı gerektiğinde (ör. bir veri yapısını birden fazla thread’in
//     okumasını istediğinizde).
// - Performans: Atomik işlemler küçük bir maliyet getirir, ancak thread güvenliği sağlar.

// `Mutex` (Mutual Exclusion) NEDİR?
// - `Mutex`, paylaşılan bir veriye yalnızca bir thread’in aynı anda erişmesini sağlar, veri
//   yarışlarını önler.
// - Nasıl Çalışır?
//   - `lock()` ile kilit alınır, bu bir `MutexGuard` döndürür.
//   - `MutexGuard`, veriye mutable erişim sağlar ve scope’tan çıktığında kilit otomatik serbest bırakılır.
//   - Diğer thread’ler, kilit serbest kalana kadar bekler.
// - Neden Kullanılır?
//   - Paylaşılan veriye mutable erişim gerektiğinde (ör. bir sayacı artırma).
// - Performans: Kilitleme/açma işlemleri overhead yaratır; yoğun kilit kullanımı performansı düşürebilir.

// `Arc` ve `Mutex` Birlikte Kullanımı
// - `Arc`, veriyi thread’ler arasında paylaşır; `Mutex`, bu veriye güvenli mutable erişim sağlar.
// - Tipik Kullanım: `Arc<Mutex<T>>` (ör. `Arc<Mutex<i32>>` bir sayacı paylaşmak için).
// - Örnek: Birden fazla thread’in aynı sayacı artırması.

// Ana fonksiyon: Tüm thread örneklerini çalıştırır
pub fn run() {
    // Örnek 1: `move` ile temel thread
    basic_thread_example();

    // Örnek 2: `move` olmadan thread (ödünç alma)
    thread_without_move_example();

    // Örnek 3: Paylaşılan veri ile çoklu thread'ler
    multiple_threads_example();

    // Örnek 4: Kapsamlı thread'ler (scoped threads)
    scoped_threads_example();

}

// Örnek 1: `move` ile temel thread
// - Basit bir thread oluşturur ve `move` kullanarak kapanışın bağımsız olmasını sağlar.
// - `join` ile thread’in tamamlanması beklenir ve hata yönetimi yapılır.
fn basic_thread_example() {
    println!("=== Temel Thread Örneği ===");
    // `move` ile bir kapanış kullanarak thread oluşturma
    // `move`, kapanışın değişkenlerin sahipliğini almasını sağlar (bu örnekte değişken yok).
    let handle = thread::spawn(move || {
        println!("Thread başladı");
        // İş simülasyonu için thread’i 1 saniye uyutuyoruz
        thread::sleep(Duration::from_secs(1));
        println!("Thread bitti");
    });

    // `join`, thread’in tamamlanmasını bekler ve bir `Result` döndürür
    match handle.join() {
        Ok(_) => println!("Thread başarıyla tamamlandı"),
        Err(e) => eprintln!("Thread panikledi: {:?}", e),
    }
}

// Örnek 2: `move` olmadan thread (ödünç alma)
// - Kapanışın değişkenleri ödünç almasını ve `move` olmadan çalışmasını gösterir.
// - `join` ile thread’in tamamlanması sağlanır, böylece ödünç alınan veri düşmeden önce
//   thread çalışır.
fn thread_without_move_example() {
    println!("\n=== `move` Olmadan Thread Örneği ===");
    let data = String::from("Merhaba, ana thread’den!");

    // `thread::scope` ile ödünç alma mümkün
    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread, ödünç alınan veriye erişiyor: {}", data);
            thread::sleep(Duration::from_secs(1));
            println!("Thread bitti");
        });
    });

    // `data` ana thread’de hala erişilebilir
    println!("Ana thread hala veriyi kullanabilir: {}", data);
}

// Örnek 3: Paylaşılan veri ile çoklu thread'ler
// - Birden fazla thread’in güvenli bir şekilde paylaşılan bir veriye (`Arc<Mutex<i32>>`)
//   erişmesini gösterir.
// - `Arc`, veriyi paylaşır; `Mutex`, güvenli mutable erişim sağlar.
fn multiple_threads_example() {
    println!("\n=== Paylaşılan Veri ile Çoklu Thread Örneği ===");
    // `Arc<Mutex<i32>>` ile paylaşılan bir sayaç oluşturuyoruz
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 5 thread oluşturuyoruz
    for i in 0..5 {
        // Her thread için `Arc`’yi klonluyoruz
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            println!("Thread {} başladı", i);
            // `Mutex`’i kilitleyerek sayaca güvenli erişim
            let mut num = counter.lock().unwrap();
            *num += 1; // Sayaç değerini artır
            thread::sleep(Duration::from_millis(100));
            println!("Thread {} bitti, sayaç: {}", i, *num);
        });
        handles.push(handle);
    }

    // Tüm thread’lerin tamamlanmasını bekliyoruz
    for handle in handles {
        handle.join().unwrap();
    }

    // Son sayaç değerini yazdır
    println!("Son sayaç değeri: {}", *shared_counter.lock().unwrap());
}

// Örnek 4: Kapsamlı thread'ler (scoped threads)
// - `thread::scope` kullanarak thread’lerin güvenli veri ödünç almasını ve kapsam
//   dışına çıkmadan tamamlanmasını gösterir.
// - Manuel `join` gerekmez; kapsam tüm thread’lerin tamamlanmasını sağlar.
fn scoped_threads_example() {
    println!("\n=== Kapsamlı Thread Örneği ===");
    let data = vec![1, 2, 3, 4, 5];

    // `thread::scope`, thread’lerin kapsam içinde çalışmasını garanti eder
    thread::scope(|s| {
        for &num in &data {
            s.spawn(move || {
                println!("Kapsamlı thread işliyor: {}", num);
                thread::sleep(Duration::from_millis(50));
            });
        }
    });

    // `data`, ödünç alındığı için hala erişilebilir
    println!("Ana thread, kapsamdan sonra veriyi kullanabilir: {:?}", data);
}


// İyileştirme Önerileri:
// 1. **Async/Await**: I/O yoğun görevler için `tokio` veya `async-std` kullanarak thread
//    oluşturma maliyetini azaltabilirsiniz.
// 2. **Thread Havuzu**: Kısa süreli görevler için `rayon` ile paralel iterasyon yapın.
// 3. **Düzgün Sonlandırma**: Thread’lerin kapanması için kanallar (channels) kullanın.
// 4. **Performans Analizi**: `perf` veya `tracing` ile thread performansını ölçün.
// 5. **Alternatif Kilitler**: `Mutex` yerine `RwLock` veya `AtomicUsize` gibi hafif
//    alternatifler deneyin.

fn main() {
    run();
}