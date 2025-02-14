// Kütüphaneleri kullanmak için modülü ekliyoruz.
// Rust'ta modül oluşturmak için mod anahtar kelimesi kullanılır.
mod data_types;

fn main() {
   
    // :: ile modülün fonksiyonlarına erişebiliriz.
    data_types::run();  // data_types modülündeki run fonksiyonunu çağırıyoruz.
}
