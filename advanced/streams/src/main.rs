// Standart giriş/çıkış işlemleri için gerekli olan io modülünü içe aktarıyoruz.
use std::io;

// io::Result<()> tipi, fonksiyonun bir giriş/çıkış işlemi sonucu başarılı veya hatalı dönebileceğini belirtir.
fn main() -> io::Result<()> {
    write_to_file()
}

// io::Result<()> tipi, fonksiyonun başarılı veya hatalı tamamlanabileceğini gösterir.
fn write_to_file() -> io::Result<()> {
    // Kullanıcıdan alınacak veriyi saklamak için boş bir String oluşturuyoruz.
    let mut input = String::new();

    // Kullanıcıya ekrana bilgi mesajı yazdırıyoruz.
    println!("Enter content to write to the file:");

    // Kullanıcıdan bir satır veri okuyoruz ve input değişkenine atıyoruz.
    // read_line fonksiyonu, kullanıcı Enter'a basana kadar bekler.
    io::stdin().read_line(&mut input)?;

    // Kullanıcının girdiği veriyi ekrana yazdırıyoruz (başındaki ve sonundaki boşlukları siliyoruz).
    println!("Writing to file: {}", input.trim());

    // Fonksiyonun başarılı şekilde tamamlandığını belirtiyoruz.
    Ok(())
}