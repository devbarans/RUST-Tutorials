use std::fmt::{self, Display}; // Formatlama için gerekli modül
use std::str::FromStr; // String'den parse için gerekli

// Ana fonksiyon: Tüm örnekler burada çalışır
pub fn run() {
    // Yeni bir User oluşturuyoruz. "Bob" otomatik olarak String'e dönüşür
    let user1 = User::new("Bob", 25);
    println!("User1: {:?}", user1); // Debug ile ham çıktı
    println!("{:#?}", user1); // Gelişmiş Debug formatı
    println!("{}", user1); // Display ile özel format

    // Varsayılan User örneği
    // Ne işe yarar? Nesneyi sıfır yapılandırma ile başlatır
    let default_user = User::default();
    println!("Default user: {:?}", default_user);

    // String'den sayı elde etme
    // Neden kullanılır? Kullanıcı girdilerini programa uygun türe çevirmek için
    let age_input = "30";
    match age_input.parse::<u8>() {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => eprintln!("Parse hatası: {}", e), // Hata stderr'e gider
    }

    // Result ve ? ile kullanıcı oluşturma
    // Teknik incelik: ? erken hata dönüşü sağlar, Result hata yönetimini kolaylaştırır
    let parsed_user = create_user_from_str("Alice:25");
    match parsed_user {
        Ok(u) => println!("Parsed user: {:?}", u),
        Err(e) => eprintln!("Hata: {}", e),
    }

    // PartialEq ile eşitlik kontrolü
    // Ne işe yarar? Nesneleri karşılaştırmak için
    let user2 = User::new("Bob", 25);
    println!("user1 == user2: {}", user1 == user2); // true döner

    // as_ref ve as_mut örnekleri
    // Neden kullanılır? Tür dönüşümünü güvenli ve esnek yapmak için
    let mut name = String::from("Dave");
    ref_example(&name); // Immutable referans ile
    mut_example(&mut name); // Mutable referans ile
    println!("Modified name: {}", name);

    // into ile tür dönüşümü
    // Teknik incelik: Manuel dönüşümden kurtarır, kodu sadeleştirir
    let user_with_into = User::new("Charlie", 40);
    println!("User with into: {:?}", user_with_into);
}

// User yapısı: Özelliklerini ve davranışlarını tanımlar
#[derive(Debug, PartialEq, Eq)] 
// Debug: Nesneyi yazdırılabilir yapar
// PartialEq: == ile eşitlik kontrolü sağlar
// Eq: Tam eşitlik için ek garanti (reflexive, symmetric, transitive)
struct User {
    name: String, // Heap'te tutulan isim
    age: u8,      // 0-255 arası yaş
}

// User metodları
impl User {
    // Yeni User oluşturur
    // impl Into<String>: &str veya String kabul eder
    // Neden kullanılır? Kullanıcıdan gelen verileri esnek bir şekilde alır
    // Teknik incelik: into ile tür dönüşümünü otomatik yapar
    // Kod Tekrarını Azaltır, Performans Artışı Sağlar
    fn new(name: impl Into<String>, age: u8) -> Self {
        User { name: name.into(), age } // into ile dönüşüm
    }
}

// Display trait'i: Özel string formatı sağlar
// Neden kullanılır? Kullanıcı dostu çıktı için
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User(name: {}, age: {})", self.name, self.age)
    }
}

// Default trait'i: Varsayılan değerler tanımlar
// Ne işe yarar? Nesneyi bilinmeyen durumlarda başlatır
impl Default for User {
    fn default() -> Self {
        User { name: String::from("Default"), age: 0 }
    }
}

// FromStr trait'i: String'den User oluşturur
// Neden kullanılır? Kullanıcı girdilerini parse etmek için
impl FromStr for User {
    type Err = String; // Hata tipi

    // from_str: String'den User'a dönüşüm
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ':' ile ayırma
        // Teknik incelik: split ile parçalama, collect ile vektöre çevirme
        let parts: Vec<&str> = s.split(':').collect();
        // Parçaların sayısını kontrol et. Hatalı formatı yakalamak için
        if parts.len() != 2 {
            return Err("Geçersiz format, 'name:age' bekleniyor".into());
        }
        // Parçaları al ve tür dönüşümü yap
        let name = parts[0].to_string();
        let age = parts[1]
            .parse::<u8>() // Yaşı u8'e çevir
            // Hata varsa döner
            // Teknik incelik: map_err ile hata mesajını özelleştir
            .map_err(|e| format!("Yaş parse hatası: {}", e))?; // ? ile erken çıkış
        Ok(User { name, age }) // Başarıyla User döner
    }
}

// Result ve ? ile yardımcı fonksiyon
// Ne işe yarar? Parse işlemini soyutlar
fn create_user_from_str(input: &str) -> Result<User, String> {
    // ? operatörü ile hata yönetimi.Hata varsa otomatik olarak döner
    // input'u User'a çevirir
    let user = input.parse::<User>()?; // FromStr'u kullanır
    Ok(user)
}

// as_ref örneği: Referans dönüşümü
// Teknik incelik: &String'i &str'ye çevirir, sahiplik almadan
fn ref_example(name: &String) {
    let name_ref: &str = name.as_ref(); // Tür açıkça belirtilir
    println!("Name as_ref: {}", name_ref);
}

// as_mut örneği: Mutable manipülasyon
// Neden kullanılır? Nesneyi değiştirmek için güvenli ödünç alma
fn mut_example(name: &mut String) {
    name.push_str(" Jr."); // Doğrudan &mut String üzerinde çalışır
    println!("Name as_mut: {}", name);
}