/* 
 Bu örnek, Rust'ta struct tanımlamaları, metot implementasyonları ve 
 trait kullanımları (özellikle Debug trait'i) ile ilgili temel konseptleri ele alır.
 Aşağıdaki kod parçası:
 - Tuple struct ve normal struct tanımlamalarını gösterir.
 - `#[derive(Debug)]` kullanımı ile struct'ların kolayca yazdırılmasını sağlar.
 - `impl` blokları ile yapıların metotlarını tanımlar.
 - Mutable referanslar, sahiplik ve metot çağrılarının nasıl çalıştığını örnekler.
 - Ek olarak, Position ve Player yapılarına yeni metotlar ekleyerek
   pratik örnekler ve kullanım senaryoları sunar.
*/
pub fn run() {
    // Dikdörtgen oluşturma ve metodlarının çağrılması
    let mut rect = Rectangle::new(10, 20);
    println!("Rectangle area: {}", rect.area());
    println!("Is square: {}", rect.is_square());
    
    // Dikdörtgenin boyutlarını değiştirme
    rect.resize(15, 15);
    println!("Rectangle resized: {}x{}", rect.width, rect.height);
    println!("Is square after resize: {}", rect.is_square());
    
    // Oyuncu oluşturma
    let player = Player {
        name: String::from("Rustacean"),
        level: 5,
        position: Position(12.0, 8.5),
    };
    println!("Player: {} (Level: {}) at position ({}, {})", player.name, player.level, player.position.0, player.position.1);
}

/// `Position` yapısı, bir nesnenin x ve y koordinatlarını temsil eden bir tuple struct'tır.
/// Genellikle oyunlar ve grafik uygulamalarında nesnelerin konumunu tutmak için kullanılır.
#[derive(Debug)] // Debug trait'ini kullanarak Position yapısını yazdırılabilir hale getirir.
struct Position(f32, f32);

/// `Player` yapısı, bir oyuncunun temel özelliklerini içerir.
/// - `name`: Oyuncunun adı
/// - `level`: Oyuncunun seviyesi (1-255 arası olabilir)
/// - `position`: Oyuncunun mevcut konumu
/// Bu yapı, oyun geliştirme ve karakter yönetim sistemlerinde kullanılabilir.
/// Debug trait'i ile yazdırılabilir hale getirilmiştir.Amaç hata ayıklamada kolaylık sağlamaktır.
#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
    position: Position,
}

/// `Rectangle` yapısı, bir dikdörtgenin genişlik ve yüksekliğini temsil eder.
/// Grafiksel uygulamalar, UI bileşenleri ve geometri hesaplamalarında kullanılır.
struct Rectangle {
    width: u32,
    height: u32,
}
// impl bloğu, bir struct için metotları tanımlamak için kullanılır.
// Rectangle struct'ı için metotlar tanımlanmıştır.
impl Rectangle {
    /// Yeni bir `Rectangle` örneği oluşturur.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    /// Dikdörtgenin alanını hesaplar.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Dikdörtgenin kare olup olmadığını kontrol eder.
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// Dikdörtgenin genişlik ve yüksekliğini değiştirmek için kullanılır.
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}