// Kütüphaneleri kullanmak için modülü ekliyoruz.
// Rust'ta modül oluşturmak için mod anahtar kelimesi kullanılır.
mod data_types;
mod vectors;
mod print_debug;
mod strings;
mod functions;
mod control_flows;
mod ownership;

// main fonksiyonu rust programlarının giriş noktasıdır.
// Rust programları çalıştırıldığında ilk olarak main fonksiyonu çalıştırılır.
// Rust'ta fonksiyonlar snake_case ile yazılır.
fn main() {
   
    // :: ile modülün fonksiyonlarına erişebiliriz.
    // Modüller çağrılırken boşluk bırakılmaz.Hataya sebep olabilir.
    data_types::run();  // data_types modülündeki run fonksiyonunu çağırıyoruz.
    vectors::run();    // vectors modülündeki run fonksiyonunu çağırıyoruz.
    print_debug::run(); // print_debug modülündeki run fonksiyonunu çağırıyoruz.
    strings::run();    // strings modülündeki run fonksiyonunu çağırıyoruz.
    functions::run(); // functions modülündeki run fonksiyonunu çağırıyoruz.
    control_flows::run(); // control_flows modülündeki run fonksiyonunu çağırıyoruz.
    ownership::run(); // ownership modülündeki run fonksiyonunu çağırıyoruz.

}
