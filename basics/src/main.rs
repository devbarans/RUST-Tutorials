// Modülleri kullanabilmek için önce onları projemize dahil ediyoruz.
// Rust'ta modül oluşturmak için `mod` anahtar kelimesi kullanılır. 
// Bu sayede programımızı daha modüler hale getirebiliriz, böylece 
// her bir işlevi ayrı dosyalarda tutarak kodun okunabilirliğini ve 
// bakımını kolaylaştırmış oluruz.

mod data_types;      // Temel veri türlerini içeren modül
mod vectors;         // Vektörler ve koleksiyonlar ile ilgili işlemleri içeren modül
mod print_debug;     // Debugging ve çıktı işlemlerini içeren modül
mod strings;         // String manipülasyonlarını içeren modül
mod functions;       // Fonksiyonlar ve fonksiyonel programlama yaklaşımlarını içeren modül
mod control_flows;   // Kontrol akışları (if, loops, match) içeren modül
mod ownership;       // Rust'ta sahiplik (ownership) ve borçlanma (borrowing) konseptlerini içeren modül
mod structs;         // Yapılar ve veri modelleri ile ilgili işlemler
mod lifetimes;       // Lifetime'lar ve bellek yönetimi ile ilgili işlemler

// `main` fonksiyonu, Rust programlarının başlama noktasını belirler.
// Program çalıştırıldığında ilk olarak `main` fonksiyonu çağrılır. 
// Rust'ta fonksiyon isimleri `snake_case` formatında yazılır, yani küçük harflerle 
// ve kelimeler arasında alt çizgi (`_`) kullanılır.

fn main() {
    // Modüllerin içindeki fonksiyonlara erişmek için `::` operatörünü kullanırız.
    // Modül ismi ve fonksiyonlar arasında boşluk bırakılmamalıdır, bu hataya yol açabilir.
    // Her bir modül, kendi fonksiyonunu çalıştırarak o modülün içerdiği işlevi yerine getirir.

    // `data_types` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon temel veri türlerini test eder.
    data_types::run();

    // `vectors` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon vektörler ile ilgili işlemleri test eder.
    vectors::run();

    // `print_debug` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon hata ayıklama (debug) işlemleri ve çıktı almayı test eder.
    print_debug::run();

    // `strings` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon string manipülasyonu ve string işlemlerini test eder.
    strings::run();

    // `functions` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Fonksiyonların nasıl kullanıldığını ve fonksiyonel programlamayı test eder.
    functions::run();

    // `control_flows` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon kontrol akışları (if, match, loops) ile ilgili işlemleri test eder.
    control_flows::run();

    // `ownership` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Rust'taki sahiplik ve borçlanma (ownership & borrowing) kavramlarını test eder.
    ownership::run();

    // `structs` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon yapıların (structs) kullanımı ve veri organizasyonunu test eder.
    structs::run();

    // `lifetimes` modülündeki `run` fonksiyonunu çalıştırıyoruz.
    // Bu fonksiyon lifetime kavramlarını ve bellek yönetimini test eder.
    lifetimes::run();
}
