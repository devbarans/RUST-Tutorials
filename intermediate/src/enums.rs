// **********************************************************************
// Rust Enum Kullanımı: Detaylı Teknik Örnek
//
// Bu dosya, Rust'ta enum'ların ve ilgili yapıların kullanımını, 
// özellikle metodlarda kullanılan &self ifadesinin işleyişini kapsamlı
// olarak açıklamaktadır.
// 
// İçerikler:
// 1. _Status Enum'ı: İşlem durumu tanımı ve &self kullanımı ile ilgili metodlar.
// 2. Level Enum ve Player Struct: Oyuncu verilerinin modellenmesi, metodlarda &self kullanımı.
// 3. User Enum: Kullanıcı durum yönetimi.
// 4. Option, if let ve Result kullanımı ile ek örnekler.
// 5. &self ifadesinin detaylı açıklaması.
// 6. Ana fonksiyon: Tüm örneklerin çalıştırılması
// **********************************************************************


// ----------------------------------------------------------------------
// 1. _Status Enum'ı: İşlem Durumları
// ----------------------------------------------------------------------
//
// _Status enum'ı, bir işlemin "başarılı" veya "başarısız" olduğunu
// ifade etmek için kullanılır. Başarısızlık durumunda hata mesajını taşır.
//
// &self Kullanımı:
// - Metod tanımında kullanılan `&self`, methodun çağrıldığı nesnenin (instance)
//   referansını alır. Bu, methodun nesnenin sahipliğini almadığı (consume etmediği)
//   ve sadece okunabilir (immutable) erişim sağladığı anlamına gelir.
// - `&self` yazımı, aslında `self: &Self` ile aynıdır. Self, bu durumda enum'un kendisini ifade eder.
enum _Status {
    Success,
    Failed(String),
}

impl _Status {
    /// _get_info metodu, enum varyantına bağlı olarak açıklayıcı bir mesaj döndürür.
    /// &self: Bu method, nesnenin kopyasını almadan, sadece ona referansla erişir.
    /// - Success durumunda "Success" döner.
    /// - Failed durumunda saklanan hata mesajını döner.
    fn _get_info(&self) -> &str {
        // match ifadesi, self referansı üzerinden enum varyantını kontrol eder.
        match self {
            _Status::Success => "Success",
            _Status::Failed(msg) => msg,
        }
    }
}

/// _create_status fonksiyonu, verilen başlığa göre _Status oluşturur.
/// Eğer başlık boşsa, Failed varyantı (hata mesajı ile) döner; aksi halde Success.
fn _create_status(title: String) -> _Status {
    if title.is_empty() {
        _Status::Failed("Title is empty".to_string())
    } else {
        _Status::Success
    }
}


// ----------------------------------------------------------------------
// 2. Level Enum ve Player Struct: Oyuncu Durumu ve Seviyesi
// ----------------------------------------------------------------------
//
// Level enum'ı, oyuncunun seviyesini belirtir.
// derive(Debug) özelliği, enum'un Debug trait'ini uygular ve println! ile yazdırılmasını sağlar.
#[derive(Debug)]
enum Level {
    Low,
    Medium,
    High,
}

// Player struct'ı, bir oyuncunun temel bilgilerini saklar.
#[derive(Debug)]
struct Player {
    name: String,
    level: Level,
    is_active: bool,
}

impl Player {
    /// new fonksiyonu, Player struct'ı için constructor görevi görür.
    /// - "Self" burada Player tipini temsil eder.
    /// - Parametreler, oyuncunun adı, seviyesi ve aktiflik durumu.
    /// - &self kullanılmaz çünkü bu method nesne oluştururken çalışır (sahipliği alır).
    fn new(name: String, level: Level, is_active: bool) -> Self {
        Player {
            name,
            level,
            is_active,
        }
    }

    /// change metodu, mevcut oyuncunun seviyesini ve aktiflik durumunu günceller.
    /// &mut self: Bu, methodun nesneyi mutable (değiştirilebilir) olarak borç aldığını belirtir.
    /// &self yerine &mut self kullanılır çünkü nesne üzerinde değişiklik yapılacaktır.
    fn change(&mut self, level: Level, is_active: bool) {
        self.level = level;
        self.is_active = is_active;
    }

    /// display metodu, oyuncu bilgilerini ekrana yazdırır.
    /// &self kullanımı, nesnenin sadece okunmasını sağlar; değişiklik yapılmaz.
    fn display(&self) {
        println!("Player: {} - Level: {:?} - Active: {}", self.name, self.level, self.is_active);
    }
}


// ----------------------------------------------------------------------
// 3. User Enum: Kullanıcı Durum Yönetimi
// ----------------------------------------------------------------------
//
// User enum'ı, bir kullanıcının aktif veya inaktif olduğunu temsil eder.
// Active varyantı ek bilgi olarak aktif gün sayısını taşır.
#[derive(Debug)]
enum User {
    Inactive { name: String },
    Active { name: String, active_days: u32 },
}


// ----------------------------------------------------------------------
// 4. Ek: Option ve if let Kullanımı
// ----------------------------------------------------------------------
//
// Option enum'ı, bir değerin varlığını veya yokluğunu ifade eder.
// Bu fonksiyon, Rust'ta Option enum'ı ile yapılan işlem örneğidir. Option,
// bir değerin olup olmadığını ifade etmek için kullanılır. Option, iki varyanta
// sahiptir: Some(T) ve None. Some(T), bir değeri temsil ederken, None, 
// değer olmadığını belirtir. `if let` ifadesi, Option değerini kontrol etmek için
// kısa ve kullanışlı bir yöntem sağlar.
// if let ifadesi, sadece Some(T) durumunu kontrol etmek için daha kısa yazım sağlar.
// Eğer Option değeri Some(T) ise, bu değeri alır ve işlem yapar. Eğer None ise, else bloğu çalışır.
// **********************************************************************

fn option_example() {
    // Option tipinde bir değişken tanımlanır ve Some değerine atanır.
    // Option<i32> tipi, i32 türünde bir değeri saklayan veya hiç değer taşımayan bir enum'dır.
    let some_number: Option<i32> = Some(42);

    // if let ifadesi, Option enum'ını kontrol etmek için kullanılır.
    // Burada, some_number değişkeni Some değeri içeriyorsa, bu değeri num değişkenine atar.
    // Eğer None değeri içeriyorsa, else bloğu çalışır.
    if let Some(num) = some_number {
        // Some değeri içeriyorsa, num değişkeni değerini bastırır.
        println!("Option contains: {}", num);
    } else {
        // None değeri içeriyorsa, Option boş olduğu mesajı bastırılır.
        println!("Option is None");
    }
}



// ----------------------------------------------------------------------
// 5. Ek: Result Kullanımı ile Hata Yönetimi
// ----------------------------------------------------------------------
//
// Result enum'ı, işlemlerin başarılı olup olmadığını ifade eder.
// - Ok(T): İşlem başarılı, sonucu içerir.
// - Err(E): İşlem başarısız, hata mesajı içerir.
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(x / y)
    }
}


// ----------------------------------------------------------------------
// 6. &self İfadesinin Detaylı Açıklaması
// ----------------------------------------------------------------------
//
// Rust'ta bir method tanımlarken, ilk parametre genellikle self, &self veya &mut self'dir.
// - `self`: Metod, nesnenin sahipliğini alır. Bu, methodun nesneyi tüketmesine (consume) neden olur.
// - `&self`: Metod, nesneye immutable (değiştirilemez) referansla erişir. Bu, nesnenin okunmasına izin verir, ancak değiştirilmesine izin vermez.
//            Bu, en yaygın kullanılan yöntemdir, çünkü nesneyi değiştirmeyecek işlemler için idealdir.
// - `&mut self`: Metod, nesneye mutable (değiştirilebilir) referansla erişir. Bu, nesne üzerinde değişiklik yapılmasına olanak tanır.
//
// Özetle, &self ifadesi:
// 1. Nesneyi borç alır, yani ownership almaz.
// 2. Sadece okunabilir erişim sağlar.
// 3. Nesnenin method çağrısından sonra hala geçerli olmasını garanti eder.
// 4. Yazım kolaylığı sağlar: &self, self: &Self'in kısaltmasıdır.
fn explain_self_usage() {
    // Bu örnek, &self kullanımını açıklamak için basit bir method içerir.
    struct Example {
        value: i32,
    }

    impl Example {
        // Bu method, &self kullanarak nesnenin değerini döndürür.
        // Nesneyi değiştirmediğimiz için &self uygundur.
        fn get_value(&self) -> i32 {
            self.value
        }
    }

    let example = Example { value: 100 };
    println!("Example value: {}", example.get_value());
}


// ----------------------------------------------------------------------
// 7. Ana Fonksiyon: run
// ----------------------------------------------------------------------
//
// run fonksiyonu, yukarıdaki tüm örnekleri çalıştırarak enum'ların, &self kullanımının,
// match, Option, Result ve if let ifadelerinin nasıl kullanıldığını gösterir.
pub fn run() {
    println!("=== Rust Enum Kullanımı ve &self Detaylı Açıklamaları ===\n");

    // _Status enum örneği
    let status1 = _create_status("Rust".to_string());
    let status2 = _create_status("".to_string());
    println!("Status1: {}", status1._get_info());
    println!("Status2: {}", status2._get_info());
    println!("");

    // Player ve Level örneği
    let mut player_001 = Player::new("Player 001".to_string(), Level::High, true);
    println!("Player before change: {:?}", player_001);
    player_001.change(Level::Medium, false);
    println!("Player after change: {:?}", player_001);
    player_001.display();  // &self kullanılan display metodu
    println!("");

    // User enum örneği
    let user = User::Active { name: "Baran".to_string(), active_days: 35 };
    match user {
        User::Active { name, active_days } => {
            println!("User {} is active for {} days", name, active_days);
        },
        User::Inactive { name } => {
            println!("User {} is inactive", name);
        },
    }
    println!("");

    // Option örneği
    option_example();
    println!("");

    // Result örneği: divide fonksiyonunu test etme
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!("");

    // &self kullanımını açıklayan örnek
    explain_self_usage();
}

