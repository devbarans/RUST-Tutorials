// Kütüphaneleri kullanmak için modülü ekliyoruz.
// Rust'ta modül oluşturmak için mod anahtar kelimesi kullanılır.
mod data_types;
mod vectors;
fn main() {
   
    // :: ile modülün fonksiyonlarına erişebiliriz.
    // Modüller çağrılırken boşluk bırakılmaz.Hataya sebeb olabilir.
    data_types::run();  // data_types modülündeki run fonksiyonunu çağırıyoruz.
    vectors::run();    // vectors modülündeki run fonksiyonunu çağırıyoruz.
}
