// Standart giriş/çıkış işlemleri için gerekli olan io modülünü içe aktarıyoruz.
use std::io::*; /// Terminalde cargo add io yazdım eğer hata alırsan kullanabilirsin
use std::net::*;
// io::Result<()> tipi, fonksiyonun bir giriş/çıkış işlemi sonucu başarılı veya hatalı dönebileceğini belirtir.
fn main() -> Result<()> {
    // _write_to_file()
    echo_stream()

}

/*
Test etmek için:
1. Terminalde `cargo run` komutunu çalıştırın.
2. Başka bir terminalde `nc localhost 7001` komutunu kullanarak sunucuya bağlanın.
*/

fn echo_stream() -> Result<()> {
    // Sunucunun dinleyeceği IP adresi ve port numarası
    // 0.0.0.0 tüm network interface'lerden gelen bağlantıları dinler
    let address = "0.0.0.0:7001";

    // TCP sunucusu oluşturup belirtilen adrese bağlanma
    // ? operatörü hata durumunda fonksiyondan erken çıkış sağlar
    let listener = TcpListener::bind(address)?;
    
    // Sunucunun başladığını kullanıcıya bildirme
    println!("Listening on {address}");

    // incoming() metodu ile gelen bağlantıları sürekli dinleme
    // Her bir bağlantı için yeni bir stream oluşturulur
    for stream in listener.incoming() {
        // stream değişkenini mutable olarak alıyoruz çünkü okuma/yazma yapacağız
        let mut stream = stream?;
        
        // 1024 byte'lık bir buffer oluşturuyoruz
        // Bu buffer'a gelen veriyi okuyacağız
        let mut buffer = [0; 1024];
        
        // Stream'den veriyi buffer'a okuma
        // bytes_read değişkeni kaç byte okunduğunu tutar
        let bytes_read = stream.read(&mut buffer)?;
        
        // Okunan veriyi UTF-8 formatında string'e çevirme
        // from_utf8_lossy geçersiz UTF-8 karakterleri için replacement character kullanır
        let message = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_owned(); // to_owned, &str tipinden String tipine dönüştürür
            
        // Alınan mesajı ekrana yazdırma
        println!("{bytes_read} bytes received: {message} ");
        
        // Alınan veriyi aynen geri gönderme (echo)
        stream.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}


// io::Result<()> tipi, fonksiyonun başarılı veya hatalı tamamlanabileceğini gösterir.
fn _write_to_file() -> Result<()> {
    // Kullanıcıdan alınacak veriyi saklamak için boş bir String oluşturuyoruz.
    let mut input = String::new();

    // Kullanıcıya ekrana bilgi mesajı yazdırıyoruz.
    println!("Enter content to write to the file:");

    // Kullanıcıdan bir satır veri okuyoruz ve input değişkenine atıyoruz.
    // read_line fonksiyonu, kullanıcı Enter'a basana kadar bekler.
    stdin().read_line(&mut input)?;

    // Kullanıcının girdiği veriyi ekrana yazdırıyoruz (başındaki ve sonundaki boşlukları siliyoruz).
    println!("Writing to file: {}", input.trim());

    // Fonksiyonun başarılı şekilde tamamlandığını belirtiyoruz.
    Ok(())
}