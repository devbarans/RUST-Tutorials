// Rust Hata Yönetimi Tutorial
// Bu doküman, Rust programlama dilinde hata yönetimini öğrenmek isteyen geliştiriciler için
// kapsamlı bir rehberdir. İleri başlangıç seviyesine yöneliktir ve şu konuları kapsar:
// 1. Unrecoverable Errors: panic! makrosu ile programın durdurulması
// 2. Recoverable Errors: Result enum'u ile kurtarılabilir hataların yönetimi
// 3. Error Propagation: ? operatörü ile hataların üst katmanlara taşınması
// 4. Özelleştirilmiş Hata Türleri: Proje ihtiyaçlarına özel hata enum'ları
// 5. İleri Seviye Teknikler: thiserror ve anyhow kütüphaneleri
// 6. Ek Özellikler: Yeniden deneme mantığı, meta veri, birim testleri
//
// Kod, gerçek dünya senaryolarını simüle eder (örneğin, kredi skoru kontrolü)
// ve modüler bir yapıya sahiptir. Her bölüm, detaylı yorumlarla açıklanmıştır.

// Gerekli kütüphaneler
use rand::Rng; // Rastgele sayı üretimi için (simülasyon amaçlı)
use std::fmt::{Display, Formatter}; // Hata mesajlarını formatlamak için
use std::fs::File; // Dosya işlemleri için
use std::io::{self, ErrorKind}; // I/O hataları için
use std::time::Duration; // Yeniden deneme süreleri için

// Sabitler: Kodun okunabilirliğini artırmak için sabitler modül başında tanımlanır
const ELIGIBLE_CREDIT_SCORE: i32 = 600; // Kredi başvurusu için minimum skor
const MAX_RETRIES: u32 = 3; // Servis yeniden deneme limiti

// Özelleştirilmiş hata türü: CreditScoreError
// Bu enum, kredi skoru kontrolü sırasında oluşabilecek hataları temsil eder
// #[derive(Debug)] ile hata ayıklama çıktıları otomatik üretilir
#[derive(Debug)]
enum CreditScoreError {
    NotFound(String), // Hesap bulunamadı, hesap ID'sini içerir
    ServiceUnavailable { retry_after: Duration }, // Servis kullanılamıyor, yeniden deneme süresi ile
    LowScore(i32), // Kredi skoru yetersiz, skoru içerir
    DatabaseError(String), // Veritabanı bağlantı hatası, hata mesajını içerir
}

// Display trait'i ile hata mesajlarını kullanıcı dostu hale getiriyoruz
// Bu, hata mesajlarının println! veya eprintln! ile düzgün görünmesini sağlar
impl Display for CreditScoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditScoreError::NotFound(account_id) => {
                // Hesap bulunamadı durumunda hesap ID'sini içeren bir mesaj
                write!(f, "Credit score not found for account ID: {}", account_id)
            }
            CreditScoreError::ServiceUnavailable { retry_after } => {
                // Servis hatası, yeniden deneme süresini içerir
                write!(
                    f,
                    "Credit service unavailable. Retry after {} seconds",
                    retry_after.as_secs()
                )
            }
            CreditScoreError::LowScore(score) => {
                // Düşük skor, minimum gereksinimi belirtir
                write!(
                    f,
                    "Credit score too low: {}. Minimum required: {}",
                    score, ELIGIBLE_CREDIT_SCORE
                )
            }
            CreditScoreError::DatabaseError(msg) => {
                // Veritabanı hatası, detaylı mesaj içerir
                write!(f, "Database error: {}", msg)
            }
        }
    }
}

// std::error::Error trait'ini uyguluyoruz
// Bu, CreditScoreError'ü diğer kütüphanelerle uyumlu hale getirir ve ? operatörü ile kullanılabilir
impl std::error::Error for CreditScoreError {
    // Varsayılan implementasyon yeterli; gerektiğinde özelleştirilebilir
}

// Kredi skoru alma fonksiyonu: Simüle edilmiş bir dış servis çağrısı
// Parametreler:
// - account_owner: Hesap sahibi (örneğin, kullanıcı ID'si)
// - retry_count: Yeniden deneme sayısı (hata durumunda tekrar denemek için)
// Dönen değer:
// - Result<i32, CreditScoreError>: Başarılıysa kredi skoru, başarısızsa hata
fn fetch_credit_score(account_owner: &str, retry_count: u32) -> Result<i32, CreditScoreError> {
    // Rastgele sayı üretecini başlatıyoruz (simülasyon için)
    let mut rng = rand::thread_rng();
    // Farklı hata senaryolarını simüle etmek için rastgele bir sayı
    let simulation = rng.gen_range(0..=4);

    match simulation {
        0 => {
            // Başarılı durum: Rastgele bir kredi skoru (300-850 arası) döndür
            let score = rng.gen_range(300..=850);
            println!("Fetched score for {}: {}", account_owner, score);
            Ok(score)
        }
        1 => {
            // Hata: Hesap bulunamadı
            Err(CreditScoreError::NotFound(account_owner.to_string()))
        }
        2 => {
            // Hata: Servis geçici olarak kullanılamıyor
            // Yeniden deneme mantığı: Maksimum deneme sayısına ulaşılmadıysa tekrar dene
            if retry_count < MAX_RETRIES {
                println!("Service unavailable, retrying... Attempt {}", retry_count + 1);
                // Simüle edilmiş bir gecikme (gerçek dünyada ağ gecikmesi gibi)
                std::thread::sleep(Duration::from_secs(1));
                fetch_credit_score(account_owner, retry_count + 1)
            } else {
                // Maksimum deneme sayısına ulaşıldı, hata döndür
                Err(CreditScoreError::ServiceUnavailable {
                    retry_after: Duration::from_secs(60),
                })
            }
        }
        3 => {
            // Hata: Veritabanı bağlantı sorunu
            Err(CreditScoreError::DatabaseError("Connection timeout".to_string()))
        }
        _ => {
            // Genel servis hatası
            Err(CreditScoreError::ServiceUnavailable {
                retry_after: Duration::from_secs(30),
            })
        }
    }
}

// Başvuru onaylama fonksiyonu: Kredi skoruna göre başvuruyu değerlendirir
// Parametre: account_owner (hesap sahibi)
// Dönen değer: Result<(), CreditScoreError> (başarılıysa boş, başarısızsa hata)
fn approve_application(account_owner: &str) -> Result<(), CreditScoreError> {
    // Kredi skorunu al, hata varsa ? operatörü ile üst katmana yay
    let score = fetch_credit_score(account_owner, 0)?;
    // Skor minimum gereksinimi karşılıyor mu?
    if score < ELIGIBLE_CREDIT_SCORE {
        // Yetersiz skor, hata döndür
        Err(CreditScoreError::LowScore(score))
    } else {
        // Başvuru onaylandı
        println!("Application approved for {}. Score: {}", account_owner, score);
        Ok(())
    }
}

// Dosya işlemleri için yardımcı fonksiyon: Recoverable hata yönetimi örneği
// Parametre: file_path (dosya yolu)
// Dönen değer: Result<File, io::Error> (başarılıysa dosya, başarısızsa I/O hatası)
fn open_or_create_file(file_path: &str) -> Result<File, io::Error> {
    // Dosyayı açmayı dene
    match File::open(file_path) {
        Ok(file) => Ok(file), // Başarılı: Dosya handle'ını döndür
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // Hata: Dosya bulunamadı, yeni dosya oluştur
            println!("File not found, creating: {}", file_path);
            File::create(file_path)
        }
        Err(e) => Err(e), // Diğer hataları üst katmana yay
    }
}

pub fn run() {
    // ANA PROGRAM AKIŞI
    // Bu bölüm, Rust'ta hata yönetiminin farklı yönlerini gösterir
    // Her alt bölüm, belirli bir hata yönetimi tekniğini açıklar

    // ==========================================================================
    // 1. Unrecoverable Errors ve panic!
    // ==========================================================================
    // Unrecoverable hatalar, programın mantıksal olarak devam edemeyeceği
    // durumlardır. Örneğin, bir dizinin sınırları dışına erişim.
    // Rust'ta bu tür hatalar panic! makrosu ile ele alınır.
    // panic! bir thread'i hata mesajıyla sonlandırır ve program genellikle çöker.
    println!("\n=== 1. Unrecoverable Errors ===");
    let numbers = vec![1, 4, 7, 0, 9];
    // Aşağıdaki satırın yorumunu kaldırırsanız, program panic! ile çöker:
    // let out_of_bounds = numbers[99];
    // Bu, "index out of bounds" hatası üretir.
    //
    // Hata izleme (backtrace):
    // Terminalde `RUST_BACKTRACE=1 cargo run` komutuyla detaylı hata izi alınabilir.
    // Bu, hatanın tam olarak nerede oluştuğunu gösterir.
    //
    // Unwinding vs. Abort:
    // - Unwinding: Varsayılan davranış. Yığın (stack) geri sarılır, kaynaklar
    //   temizlenir. Geliştirme sırasında hata ayıklamayı kolaylaştırır, ancak
    //   daha fazla bellek kullanır.
    // - Abort: Program anında sonlanır, kaynak temizliği yapılmaz. Performans
    //   odaklıdır ve üretim ortamında tercih edilebilir.
    //   Ayar: Cargo.toml'da `[profile.release] panic = "abort"`
    // Öneri: Geliştirme için unwinding, üretim için abort kullanın.
    println!("Unrecoverable error example skipped (commented out).");

    // ==========================================================================
    // 2. Recoverable Errors ve Result
    // ==========================================================================
    // Recoverable hatalar, programın devam edebileceği durumlardır.
    // Örneğin, bir dosyanın bulunmaması veya bir ağ isteğinin başarısız olması.
    // Rust, bu tür hataları Result<T, E> enum'u ile yönetir:
    // - Ok(T): Başarılı sonuç
    // - Err(E): Hata
    println!("\n=== 2. Recoverable Errors ===");
    // Örnek: Dosya açma/kurma
    match open_or_create_file("there_is_no_spoon.dat") {
        Ok(file) => println!("File opened/created successfully: {:?}", file),
        Err(e) => eprintln!("Failed to handle file: {}", e),
    }
    // open_or_create_file fonksiyonu:
    // - Dosyayı açmayı dener.
    // - Dosya yoksa (ErrorKind::NotFound), yeni bir dosya oluşturur.
    // - Diğer hataları üst katmana yayar.
    //
    // Result ile çalışmanın yöntemleri:
    // - match: Hata türlerine göre dallanma sağlar, en güvenli yöntemdir.
    // - unwrap(): Hata varsa panic! üretir. Hızlı prototipleme için uygundur,
    //   ancak üretimde risklidir.
    // - expect(): unwrap gibi, ancak özel bir hata mesajı ekler.
    // - unwrap_or_else(): Hata durumunda yedek bir davranış sağlar.
    // Örnek:
    // let file = File::open("example.txt").expect("Failed to open example.txt");
    // Yukarıdaki gibi expect kullanımı, hata mesajını özelleştirir.
    println!("Recoverable error handling demonstrated with file operations.");

    // ==========================================================================
    // 3. Error Propagation
    // ==========================================================================
    // Hata yayılımı, bir fonksiyondaki hatayı çağıran fonksiyona aktarmaktır.
    // Rust'ta bu, genellikle ? operatörü ile yapılır.
    // ? operatörü:
    // - Ok(T) ise değeri döndürür.
    // - Err(E) ise fonksiyondan erken dönüş yapar ve hatayı üst katmana yayar.
    println!("\n=== 3. Error Propagation ===");
    // Örnek: Birden fazla kredi başvurusu işleme
    let applicants = vec!["Alice", "Bob", "Charlie"];
    for applicant in applicants {
        match approve_application(applicant) {
            Ok(_) => println!("{}'s application approved.", applicant),
            Err(e) => eprintln!("Error for {}: {}", applicant, e),
        }
    }
    // approve_application fonksiyonu:
    // - fetch_credit_score'dan kredi skorunu alır.
    // - Hata varsa, ? operatörü ile hatayı main'e yayar.
    // - Skor yetersizse, LowScore hatası üretir.
    //
    // ? operatörünün avantajları:
    // - Kod daha kısa ve okunabilir olur.
    // - Hata türleri arasında dönüşüm yapmayı kolaylaştırır (From trait'i ile).
    // Örnek (kodda uygulanmadı, ancak mümkün):
    // impl From<io::Error> for CreditScoreError { ... }
    // Bu, io::Error'ü otomatik olarak CreditScoreError'e çevirebilir.
    println!("Error propagation demonstrated with application approvals.");

    // ==========================================================================
    // 4. Özelleştirilmiş Hata Türleri
    // ==========================================================================
    // Özelleştirilmiş hata türleri, proje ihtiyaçlarına özel hataları tanımlamak
    // için kullanılır. CreditScoreError enum'u buna bir örnektir.
    println!("\n=== 4. Custom Error Types ===");
    // CreditScoreError özellikleri:
    // - NotFound: Hesap bulunamadı, hesap ID'sini içerir.
    // - ServiceUnavailable: Servis hatası, retry_after meta verisi ile.
    // - LowScore: Düşük kredi skoru, skoru içerir.
    // - DatabaseError: Veritabanı bağlantı hatası, hata mesajını içerir.
    //
    // Trait'ler:
    // - Display: Kullanıcı dostu hata mesajları üretir.
    // - std::error::Error: Hata türünü diğer kütüphanelerle uyumlu hale getirir.
    //
    // Örnek bir hata durumu:
    if let Err(e) = fetch_credit_score("TestUser", 0) {
        println!("Custom error example: {}", e);
    }
    // Gerçek dünyada özelleştirilmiş hatalar:
    // - API yanıtlarındaki hata kodlarını temsil edebilir.
    // - Veritabanı veya ağ hatalarını kapsayabilir.
    // - İş mantığına özel durumları ifade edebilir (örneğin, LowScore).
    //
    // Meta veri kullanımı:
    // ServiceUnavailable'daki retry_after, gerçek dünyada istemcilerin
    // ne zaman yeniden deneme yapacağını belirtebilir.
    println!("Custom error types demonstrated with credit score errors.");

    // ==========================================================================
    // 5. İleri Seviye: thiserror ve anyhow
    // ==========================================================================
    // Gerçek projelerde, hata türlerini yönetmek için kütüphaneler kullanılır.
    // Bu bölüm, kodda uygulanmadı, ancak yorumlarla detaylıca açıklanıyor.
    println!("\n=== 5. Advanced: thiserror and anyhow ===");
    //
    // a) thiserror:
    // - Özelleştirilmiş hata türleri için derive makroları sağlar.
    // - Daha az kodla Display ve Error trait'lerini uygular.
    // - Kullanım senaryosu: Büyük projelerde, hata türlerini yapılandırmak için.
    // - Örnek kullanım:
    // #[derive(thiserror::Error, Debug)]
    // enum MyError {
    //     #[error("An error occurred: {0}")]
    //     CustomError(String),
    //     #[error("Another error occurred")]
    //     AnotherError,
    // }
    //
    // b) anyhow:
    // - Genel hata türleri için kullanılır.
    // - Hata türlerini otomatik olarak kapsar.
    // - Hata mesajlarını ve kaynakları yönetir.
    // - Kullanım senaryosu: Hızlı prototipleme ve hata yönetimi için.
    // - Örnek kullanım:
    // fn do_something() -> anyhow::Result<()> {
    //     // Hata durumunda otomatik olarak anyhow::Error döner.
    //     Ok(())
    // }
    //
    // c) Hangi kütüphaneyi seçmeli?
    // - Projenin karmaşıklığına bağlıdır.
    // - Küçük projelerde anyhow yeterli olabilir.
    // - Büyük projelerde thiserror ile özelleştirilmiş hata türleri tercih edilebilir.
    //
    // ==========================================================================
    // 6. Ek Özellikler
    // ==========================================================================
    // a) Yeniden deneme mantığı:
    // - Hata durumunda belirli bir süre bekleyip tekrar deneme.
    // - Örnek: Servis hatası durumunda retry_after meta verisi ile.
    // - Gerçek dünyada ağ gecikmeleri veya geçici hatalar için kullanılır.
    // - Kodda uygulanmadı, ancak yukarıdaki örneklerde gösterildi.
    //
    // b) Meta veri:
    // - Hata türlerine ek bilgi eklemek için kullanılır.
    // - Örnek: ServiceUnavailable'daki retry_after.
    // - Gerçek dünyada hata türlerini daha anlamlı hale getirir.
    // - Kodda uygulanmadı, ancak yukarıdaki örneklerde gösterildi.
    //
    // c) Birim testleri:
    // - Hata türlerini ve fonksiyonları test etmek için kullanılır.
    // - Rust'ta test modülü ile birim testleri yazılır.
    // - Örnek: fetch_credit_score fonksiyonunu test etmek için.
    // - Kodda uygulanmadı, ancak test senaryoları yazılabilir.
    // - Testler, hata türlerinin doğru çalıştığını doğrular.
}