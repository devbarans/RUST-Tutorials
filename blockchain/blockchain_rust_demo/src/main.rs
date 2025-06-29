// Rust ile basit bir blockchain simülasyonu
// Konular: Blok oluşturma, hash hesaplama, PoW madenciliği, zincir doğrulama

use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, str};
use sha2::{Sha256, Digest};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

// Hash'in başlaması gereken zorluk hedefi
const DIFFICULTY: usize = 0x1d00ffff;

// Bir blok yapısı tanımlanıyor
struct Block {
    index: u32,                      // Blok numarası (örneğin 0, 1, 2...)
    previous_block_hash: String,    // Önceki bloğun hash değeri (zinciri kurmak için)
    timestamp: u64,                 // Blok oluşturulma zamanı (Unix timestamp)
    data: String,                   // Blok içinde saklanan veri (örnek: işlem bilgisi)
    nonce: u32,                     // Doğru hash'i bulana kadar artan sayı (madencilikte kullanılır)
    hash: String,                   // Bu bloğun hash sonucu (SHA256 ile)
}

impl Block {
    // Yeni bir blok oluşturur (henüz madencilik yapılmaz, hash boş)
    fn new(index: u32, previous_block_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,
            previous_block_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    // Blok içeriğini SHA256 ile hash'ler, nonce dahil edilir
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        // Hash girişine sırayla tüm veriler eklenir
        hasher.update(self.index.to_string());
        hasher.update(&self.previous_block_hash);
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.data);
        hasher.update(self.nonce.to_string());
        // Hex formatında string olarak hash döndürülür
        format!("{:x}", hasher.finalize())
    }

    // Doğru hash değerini bulana kadar nonce artırarak madencilik yapar
    fn mine(&mut self) {
        loop {
            self.hash = self.calculate_hash(); // Hash hesaplanır
            if self.is_valid() {              // Eğer hedef zorluğu sağlıyorsa dur
                break;
            }
            self.nonce += 1;                   // Aksi halde nonce'u artır, yeniden dene
            thread::sleep(Duration::from_millis(10)); // Gerçekçi madencilik hissi için bekleme
        }
    }

    // Hash, belirlenen zorluk koşulunu sağlıyor mu kontrol edilir
    fn is_valid(&self) -> bool {
        let target = DIFFICULTY.to_string();
        self.hash.starts_with(&target) // Gerçek sistemlerde bu daha karmaşıktır (örneğin 4 sıfırla başlama)
    }

    // Blok bilgileri ekrana yazdırılır
    fn display(&self) {
        println!("Block Index: {}", self.index);
        println!("Previous Block Hash: {}", self.previous_block_hash);
        println!("Timestamp: {}", self.timestamp);
        println!("Data: {}", self.data);
        println!("Nonce: {}", self.nonce);
        println!("Hash: {}", self.hash);
    }
}

// Blockchain yapısı: Bloklardan oluşan bir vektör
struct Blockchain {
    blocks: Vec<Block>, // Blokları tutan dizi
}

impl Blockchain {
    // Boş bir blockchain başlatılır (henüz genesis block yok)
    fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }

    // Zincire yeni bir blok ekler (veri girilir, önceki hash'e bağlanır, madencilik yapılır)
    fn add_block(&mut self, data: String) {
        let index = self.blocks.len() as u32; // Yeni bloğun indeksi
        let previous_block_hash = if index == 0 {
            String::from("0") // İlk bloksa (genesis block), önceki hash "0" kabul edilir
        } else {
            self.blocks[index as usize - 1].hash.clone() // Önceki bloğun hash değeri alınır
        };
        let mut block = Block::new(index, previous_block_hash, data); // Blok oluşturulur
        block.mine(); // Madencilik yapılır (hash bulunana kadar nonce denenir)
        self.blocks.push(block); // Zincire eklenir
    }

    // Tüm bloklar terminalde gösterilir
    fn display(&self) {
        for block in &self.blocks {
            block.display();
            println!("-------------------------");
        }
    }

    // Zincirin geçerliliği kontrol edilir (tüm bloklar doğru hash'e ve bağlantıya sahip mi?)
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            // Önceki hash doğru mu?
            if current_block.previous_block_hash != previous_block.hash {
                return false;
            }

            // Hash yeniden hesaplandığında aynı mı?
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
        }
        true
    }
}

// Uygulamanın ana fonksiyonu - kullanıcı arayüzü gibi çalışır
fn main() {
    println!("=== Blockchain Simulator ===");
    let mut blockchain = Blockchain::new(); // Yeni blockchain başlatılır

    loop {
        // Menü seçenekleri gösterilir
        println!("\nAvailable commands:");
        println!("1. Add new transaction");
        println!("2. Display blockchain");
        println!("3. Verify blockchain");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                // Yeni işlem verisi istenir ve blok olarak zincire eklenir
                print!("Enter transaction data: ");
                io::stdout().flush().unwrap();
                let mut data = String::new();
                io::stdin().read_line(&mut data).expect("Failed to read input");
                println!("Mining block...");
                blockchain.add_block(data.trim().to_string());
                println!("Block mined and added to chain!");
            }
            "2" => {
                // Zincir görüntülenir
                println!("\nBlockchain contents:");
                blockchain.display();
            }
            "3" => {
                // Zincirin geçerli olup olmadığı kontrol edilir
                println!("\nVerifying blockchain integrity...");
                if blockchain.is_chain_valid() {
                    println!("Blockchain is valid!");
                } else {
                    println!("Blockchain validation failed!");
                }
            }
            "4" => {
                // Programdan çıkış yapılır
                println!("Exiting blockchain simulator...");
                break;
            }
            _ => println!("Invalid choice, please try again."), // Geçersiz seçim için uyarı
        }
    }
}
