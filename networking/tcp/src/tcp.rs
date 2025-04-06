// Importing necessary modules from the rust library

// io and net modules are used for input/output operations and network communication
// io ve net modülleri giriş/çıkış işlemleri ve ağ iletişimi için kullanılır
use std::net::TcpStream;
use std::io::{Read, Write};


// handle_client function is used to handle a single TCP connection
// handle_client fonksiyonu, tek bir TCP bağlantısını işlemek için kullanılır
pub fn handle_client(mut stream: TcpStream) {

    // This is a buffer to temporarily store data read from the client
    // Bu, istemciden okunan veriyi geçici olarak depolamak için bir tampon (buffer)
    // It is a 1024-element array where each element is initialized to 0
    // Bu, her elemanı 0 olarak başlatılmış 1024 elemanlı bir dizidir
    let mut buffer = [0; 1024]; 

    // This line reads data from the TCP stream and writes it into the buffer
    // Bu satır, TCP akışından veri okur ve buffer dizisine yazar
    // If the read operation fails, the program will panic with the given error message
    // Eğer okuma işlemi başarısız olursa, program belirtilen hata mesajıyla durur
    stream.read(&mut buffer).expect("Failed to read from client"); 

    // Converts the byte array (buffer) into a UTF-8 string
    // Byte dizisini (buffer) UTF-8 formatında bir metne dönüştürür
    // If the byte array contains invalid UTF-8, it replaces invalid characters with �
    // Eğer byte dizisi geçersiz UTF-8 içeriyorsa, geçersiz karakterleri � ile değiştirir
    /*
        -> lossy_utf8, bir byte dilimini UTF-8 metnine dönüştüren bir yöntemdir.
        Geçersiz UTF-8 dizilerini � (U+FFFD) ile değiştirir
        -> &buffer[..] ifadesi, buffer dizisinin tamamını alır
     */
    // At this point, `request` contains the data sent by the client as a readable string
    // Bu noktada, `request` istemcinin gönderdiği veriyi okunabilir bir metin olarak içerir
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    // Converts the response string into a byte array to send over the TCP connection
    // Yanıt metnini TCP bağlantısı üzerinden göndermek için bir byte dizisine dönüştürür
    let response = "Hello, Client".as_bytes(); 

    // This line writes the response byte array back to the TCP stream
    // Bu satır, yanıt byte dizisini TCP akışına geri yazar
    stream.write(response).expect("Failed to write to response");
    
    
}