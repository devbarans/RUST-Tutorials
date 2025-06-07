// log! deklaratif macro: Mesajları konsola ve dosyaya yazar
// Mantık: Pattern matching ile mesaj türlerini (info, error) işler
// Ne işe yarar? Loglama kodlarını tekrar yazmayı önler
// Teknik incelik: Derleme zamanında genişler, runtime maliyeti sıfır
// Ne zaman kullanılır? Basit, tekrar eden kodları otomatikleştirmek için
// Performans: Derleme süresi biraz artar, runtime'da sıfır ek yük
// Yazım: macro_rules! ile pattern ($msg:expr) tanımlanır, export için #[macro_export]
// Sektörde println! veya loglama gibi tekrar eden görevlerde kullanılır

use std::io::Write; // write_all ve flush gibi metodları kullanabilmek için bu trait'i içe aktarıyoruz

// Makroyu dışa aktarmak için kullanılır. Böylece başka modüllerden de bu makro çağrılabilir.
// Eğer sadece aynı dosyada kullanacaksan #[macro_export] yazmak şart değildir.
#[macro_export]
macro_rules! log {
    // İlk pattern: log!(info, "mesaj") şeklindeki kullanım için geçerlidir
    //
    // Açıklama:
    // - "info" sabit bir kelimedir, makro çağrısında bu tam olarak eşleşmelidir
    // - $msg:expr ifadesi, herhangi bir geçerli Rust ifadesini yakalar (örneğin bir string literal)
    //
    // => ile eşleşen pattern için yapılacak işlemler tanımlanır
    (info, $msg:expr) => {
        // stdout (normal terminal) ekranına mesajı yazdırıyoruz
        // println! makrosu zaten Rust içinde tanımlı bir makrodur
        println!("[INFO] {}", $msg);

        // Aşağıda log mesajını dosyaya yazmak için bir dosya açıyoruz
        // OpenOptions ile:
        // - .create(true): Dosya yoksa oluştur
        // - .append(true): Mevcut dosyanın sonuna ekle
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log") // app.log adında bir dosya açılır/yazılır
            .expect("failed to open log file"); // Dosya açılmazsa hata mesajı gösterilir

        // Mesajı "[INFO] ..." formatında oluşturuyoruz ve sonuna newline (\n) ekliyoruz
        let formatted = format!("[INFO] {}\n", $msg);

        // Dosyaya yazma işlemi — byte dizisi olarak yazılır
        file.write_all(formatted.as_bytes()).expect("failed to write to log file");

        // Yazma işlemini garantiye almak için buffer'ı flush ediyoruz (boşaltma)
        file.flush().expect("failed to flush log file");
    };

    // İkinci pattern: log!(error, "mesaj") şeklindeki kullanım için geçerlidir
    //
    // Bu pattern hata mesajları için özel olarak tanımlanmıştır.
    (error, $msg:expr) => {
        // eprintln! ile stderr (hata çıktısı) kanalına kırmızı renkte mesaj yazdırıyoruz
        // \x1b[31m = kırmızı başlat, \x1b[0m = renk sıfırla
        eprintln!("\x1b[31m[ERROR] {}\x1b[0m", $msg);

        // Aynı şekilde, hata mesajını da log dosyasına ekliyoruz
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("app.log")
            .expect("failed to open log file");

        // "[ERROR] ..." formatında string oluşturuluyor
        let formatted = format!("[ERROR] {}\n", $msg);

        // Dosyaya yazılıyor
        file.write_all(formatted.as_bytes()).expect("failed to write to log file");

        // Flush işlemi ile tampon boşaltılıyor
        file.flush().expect("failed to flush log file");
    };
}

// Ana fonksiyon — makroyu test etmek için kullanılır
fn main() {
    // Info türünde bir mesaj yazdırıyoruz
    // Beklenen çıktı:
    // Terminal: [INFO] application started
    // app.log dosyasına da aynı satır yazılır
    log!(info, "application started");

    // Error türünde bir mesaj yazdırıyoruz
    // Beklenen çıktı:
    // Terminal (kırmızı): [ERROR] failed to connect to database
    // app.log dosyasına da aynı satır yazılır
    log!(error, "failed to connect to database");
}
