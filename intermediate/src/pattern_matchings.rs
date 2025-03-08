/*
   Rust Pattern Matching Tutorial: Gerçek Dünya Senaryoları
   -----------------------------------------------------------
   Bu örnekte, Rust'ın pattern matching özelliklerini gerçek hayattan
   örneklerle detaylı olarak inceliyoruz. Kullanıcı rolleri, API yanıtları,
   Option ve Result kullanımları, hesap yönetimi ve HTTP durum simülasyonu gibi 
   konuları kapsayan bu kod, teknik açıdan dolu ve akılda kalıcı bir tutorial niteliğindedir.
*/

use rand::Rng; // Rastgele sayı üretimi için

// -------------------------------------------------------
// Ana Fonksiyon: Tüm Fonksiyonların Çalıştırılması
// -------------------------------------------------------
pub fn run() {
    println!("=== Rust Pattern Matching ve Gerçek Dünya Senaryoları ===\n");
    
    // 1. Kullanıcı Rolleri: Yetkilendirme kontrolü
    println!("-- Kullanıcı Rolleri ve Yetkilendirme --");
    let role_admin = UserRole::Admin;
    let role_manager = UserRole::Manager("Finans".to_string());
    let role_user = UserRole::User { id: 101, name: "Baran".to_string() };
    check_permissions(&role_admin);
    check_permissions(&role_manager);
    check_permissions(&role_user);
    
    // 2. API Yanıtları: Çeşitli durumların işlenmesi
    println!("\n-- API Yanıt İşlemleri --");
    handle_response(ApiResponse::Success("Veri başarıyla alındı.".to_string()));
    handle_response(ApiResponse::Error { code: 404, message: "Sayfa bulunamadı.".to_string() });
    handle_response(ApiResponse::NotFound);
    
    // 3. Option Kullanımı: E-posta kontrolü
    println!("\n-- Option ile E-posta Kontrolü --");
    print_email(1); // Kullanıcı ID 1: e-posta mevcut.
    print_email(2); // Diğer ID: e-posta yok.
    
    // 4. Result Kullanımı: Güvenli bölme işlemleri
    println!("\n-- Result ile Bölme İşlemleri --");
    process_division(10.0, 2.0);  // Normal bölme işlemi
    process_division(10.0, 0.0);  // Sıfıra bölme hatası
    
    // 5. Kullanıcı Bilgileri: Kullanıcı nesnesi oluşturma ve bilgi gösterimi
    println!("\n-- Kullanıcı Bilgileri --");
    let user = User::new(1, "Baran".to_string(), Some("baran@example.com".to_string()));
    println!("Kullanıcı Bilgileri: {}", user.info());
    
    // 6. Hesap Yönetimi: Hesap arama ve gösterim
    println!("\n-- Hesap Yönetimi --");
    let accounts = load_accounts();
    if let Some(account) = find_account(&accounts, 1003) {
        println!("Hesap Bulundu: {} - {} | Bakiye: ${}", account.id, account.holder_name, account.balance);
    } else {
        println!("Aranan hesap bulunamadı.");
    }
    
    // 7. HTTP Durum Simülasyonu: Rastgele HTTP durum kodu üretimi
    println!("\n-- HTTP Durum Simülasyonu --");
    let status = ping("http://localhost:3456/ping");
    match status {
        HttpStatus::Ok => println!("HTTP Durum: Ok (200)"),
        HttpStatus::Accepted => println!("HTTP Durum: Accepted (201)"),
        HttpStatus::NotFound => println!("HTTP Durum: Not Found (404)"),
        HttpStatus::BadRequest => println!("HTTP Durum: Bad Request (400)"),
        HttpStatus::InternalServerError => println!("HTTP Durum: Internal Server Error (500)"),
    }
    
    println!("\n=== Tüm Senaryolar Başarıyla Çalıştı ===");
}


// -------------------------------------------------------
// 1. Kullanıcı Rolleri ve Yetkilendirme (UserRole)
// -------------------------------------------------------
#[derive(Debug)]
enum UserRole {
    Admin,
    Manager(String), // Yöneticiler: bağlı oldukları departman bilgisi
    User { id: u32, name: String }, // Normal kullanıcılar: id ve isim bilgisi
}

// Kullanıcı rolüne göre yetkilendirme kontrolü yapan fonksiyon.
fn check_permissions(role: &UserRole) {
    match role {
        UserRole::Admin => println!("Yetki: Tam erişim (Admin)"),
        UserRole::Manager(department) => {
            println!("Yetki: {} departmanına özel erişim (Manager)", department)
        },
        UserRole::User { id, name } => {
            println!("Yetki: {} (ID: {}) için sınırlı erişim (User)", name, id)
        },
    }
}

// -------------------------------------------------------
// 2. API Yanıtları (ApiResponse)
// -------------------------------------------------------
#[derive(Debug)]
enum ApiResponse {
    Success(String),
    Error { code: u16, message: String },
    NotFound,
}

// API yanıtlarını işleyen fonksiyon.
fn handle_response(response: ApiResponse) {
    match response {
        ApiResponse::Success(data) => println!("İşlem Başarılı: {}", data),
        ApiResponse::Error { code, message } => println!("Hata {}: {}", code, message),
        ApiResponse::NotFound => println!("Hata: Kaynak bulunamadı"),
    }
}

// -------------------------------------------------------
// 3. Option Kullanımı: E-posta Kontrolü (get_email, print_email)
// -------------------------------------------------------
fn get_email(user_id: u32) -> Option<String> {
    // Örnek senaryo: user_id 1 olan kullanıcıya e-posta atanmış
    if user_id == 1 {
        Some("user@example.com".to_string())
    } else {
        None
    }
}

// Kullanıcının e-posta adresini yazdıran fonksiyon.
fn print_email(user_id: u32) {
    match get_email(user_id) {
        Some(email) => println!("E-posta: {}", email),
        None => println!("Bu kullanıcı için e-posta bulunamadı"),
    }
}

// -------------------------------------------------------
// 4. Result Kullanımı: Güvenli Bölme İşlemi (safe_divide, process_division)
// -------------------------------------------------------
fn safe_divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Sıfıra bölme hatası".to_string())
    } else {
        Ok(x / y)
    }
}

// Bölme işlemi sonucunu işleyen fonksiyon.
fn process_division(x: f64, y: f64) {
    match safe_divide(x, y) {
        Ok(result) => println!("Bölme Sonucu: {}", result),
        Err(err) => println!("Bölme Hatası: {}", err),
    }
}

// -------------------------------------------------------
// 5. Kullanıcı Bilgileri (User Struct)
// -------------------------------------------------------
struct User {
    id: u32,
    name: String,
    email: Option<String>, // E-posta opsiyonel: Some veya None olabilir
}

impl User {
    // Yeni bir kullanıcı oluşturur.
    fn new(id: u32, name: String, email: Option<String>) -> Self {
        User { id, name, email }
    }

    // Kullanıcı bilgilerini döndürür; e-posta varsa gösterir.
    fn info(&self) -> String {
        match &self.email {
            Some(email) => format!("{} - {} ({})", self.id, self.name, email),
            None => format!("{} - {}", self.id, self.name),
        }
    }
}

// -------------------------------------------------------
// 6. Hesap Yönetimi: Hesap Bulma ve İşlemler (Account)
// -------------------------------------------------------
struct Account {
    id: u32,
    holder_name: String,
    balance: f64,
}

// Hesap listesinden belirli bir ID'ye sahip hesabı bulan fonksiyon.
fn find_account(accounts: &Vec<Account>, id: u32) -> Option<&Account> {
    accounts.iter().find(|acc| acc.id == id)
}

// Örnek hesapları yükleyen fonksiyon.
fn load_accounts() -> Vec<Account> {
    vec![
        Account { id: 1001, holder_name: "Nora Min".to_string(), balance: 1000.0 },
        Account { id: 1002, holder_name: "Agnis Yang".to_string(), balance: 750.0 },
        Account { id: 1003, holder_name: "Valeri Mora".to_string(), balance: 850.0 },
        Account { id: 1004, holder_name: "Monti Konti".to_string(), balance: 275.0 },
    ]
}

// -------------------------------------------------------
// 7. Rastgele HTTP Durumlarını Simüle Eden Fonksiyon (ping)
// -------------------------------------------------------
#[derive(Debug)]
enum HttpStatus {
    Ok,
    Accepted,
    NotFound,
    BadRequest,
    InternalServerError,
}

// Rastgele bir HTTP durumunu simüle eden fonksiyon.
// Gerçek bir sağlık kontrolü çağrısına benzetilmiştir.
fn ping(service_url: &str) -> HttpStatus {
    let mut rng = rand::rng();
    let random_value = rng.random_range(1..=10);
    println!("{} adresine ping atılıyor...", service_url);
    
    match random_value {
        1 => HttpStatus::Ok,
        2..=4 => HttpStatus::Accepted,
        5 => HttpStatus::BadRequest,
        8 | 10 => HttpStatus::NotFound,
        _ => HttpStatus::InternalServerError,
    }
}