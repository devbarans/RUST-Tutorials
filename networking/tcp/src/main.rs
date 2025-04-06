
/*
Bir şeyin dışarıdan görünür olmasını	pub ile tanımlarsın
Modül dosyasını tanıtmak	mod tcp;
Fonksiyonu içeri almak	use tcp::handle_client;
*/

mod tcp;

use std::net::TcpListener;
use tcp::handle_client; // BU SATIR ÖNEMLİ ; Böylece handle_client fonksiyonunu kullanabiliriz
fn main() {
    // This line creates a TCP listener that binds to the specified address and port
    // Bu satır, belirtilen adres ve bağlantı noktasına bağlanan bir TCP dinleyici oluşturur
    let listener = TcpListener::bind("127.0.0.1:8080").
    expect("Failed to bind to address");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        // This line handles each incoming TCP connection in a separate thread
        // Bu satır, her gelen TCP bağlantısını ayrı bir iş parçacığında işler
        match stream {
            Ok(stream) => {
            /*
               std::thread::spawn yeni bir iş parçacığı (thread) başlatır.
               Bu, programın aynı anda birden fazla işi paralel olarak yapmasını sağlar.
               Örneğin, her istemci bağlantısı için ayrı bir iş parçacığı oluşturmak gibi.

               Closure `|| handle_client(stream)` ise handle_client fonksiyonunu çağırır.
               Closure, bir anonim fonksiyondur ve `||` ile tanımlanır.
               Bu closure, yeni iş parçacığında çalıştırılacak işlemi temsil eder.
               Burada `handle_client(stream)` fonksiyonu, istemci bağlantısını işlemek için çağrılır.
            */
                std::thread::spawn(|| handle_client(stream));



            }
            Err(e) => {
                // eprintln ile hata mesajını standart hata akışına yazdırır (stderr)
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}