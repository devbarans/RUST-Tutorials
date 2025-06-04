// basic modülünü tanımlıyoruz, basic.rs dosyasından gelir
mod basic;
//mod scenarios;
// modülden fonksiyonları kullanmak için
use crate::basic::{hello_channels, multi_producer_example,multi_producer_2};
//use crate::scenarios::*;


// Ana fonksiyon: Programın giriş noktası
// Ne işe yarar? MPSC kanal örneklerini çalıştırır ve çıktıları organize eder
// #[tokio::main]
fn main() {
    // MPSC kanal örneğini çalıştır
    println!("=== MPSC Kanal Örneği ===");
    hello_channels();

    // Multi-producer örneğini çalıştır
    println!("\n=== Multi-Producer Örneği ===");
    multi_producer_example();

    // Multi-producer 2 örneğini çalıştır
    println!("\n=== Multi-Producer 2 Örneği ===");
    multi_producer_2();
/* 
    // Process reports örneğini çalıştır
    println!("\n=== Process Reports Örneği ===");
    process_reports();

    // Do with standard örneğini çalıştır
    println!("\n=== Do With Standard Örneği ===");
    do_with_standard();

   
    println!("\n=== Do With Tokio Örneği ===");
    // Tokio fonksiyonu async, bu yüzden runtime içinde çalıştırılır
    do_with_tokio().await;
*/

}