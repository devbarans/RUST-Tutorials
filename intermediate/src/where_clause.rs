/*
RUST'ta `where` Anahtar Kelimesi – Profesyonel Kullanım Rehberi

`where` ifadesi, generic fonksiyon, struct veya trait tanımlarında çok sayıda trait bound (özellik sınırlaması) olduğunda,
kodu daha okunabilir ve profesyonel hale getirmek için kullanılır.

Özellikler:
- Trait sınırlarını (`T: Trait1 + Trait2`) fonksiyonun imza satırından ayırır.
- Birden fazla generic parametre olduğunda kod karmaşasını azaltır.
- `impl Trait`, lifetime, associated type içeren durumlarda neredeyse zorunludur.

Söz Dizimi:
fn function_name<T, U>(params...) 
where 
    T: Trait1 + Trait2,
    U: Trait3,
{
    // ...
}

*/

use std::fmt::{Display, Debug};

// Kötü Kullanım (Inline Trait Bound) – okunabilirlik bozuk
fn log_items_inline<T: Display + Debug, U: Display + Debug>(item1: T, item2: U) {
    println!("Item 1 => {}, {:?}", item1, item1);
    println!("Item 2 => {}, {:?}", item2, item2);
}

// Temiz Kullanım (`where` ile) – okunabilir ve sektör standardı
fn log_items<T, U>(item1: T, item2: U)
where
    T: Display + Debug,
    U: Display + Debug,
{
    println!("Item 1 => {}, {:?}", item1, item1);
    println!("Item 2 => {}, {:?}", item2, item2);
}

// Trait Bound içeren Struct
#[derive(Debug)]
struct Report<T, U>
where
    T: Display + Debug,
    U: Debug,
{
    title: T,
    data: U,
}

impl<T, U> Report<T, U>
where
    T: Display + Debug,
    U: Debug,
{
    fn print(&self) {
        println!("Report: {}", self.title);
        println!("Data: {:?}", self.data);
    }
}

// where ile Closure Parametresi Alan Fonksiyon
fn call_with_callback<F>(callback: F)
where
    F: Fn(i32),
{
    callback(100);
}

// Gerçek Hayat Senaryosu: Web API log fonksiyonu
fn log_request<T, B>(endpoint: T, body: B)
where
    T: AsRef<str>,
    B: Debug + Send + 'static,
{
    println!("API Endpoint: {}", endpoint.as_ref());
    println!("Request Body: {:?}", body);
}

pub fn run() {
    println!("== where keyword örnekleri ==");

    // Inline bound vs. where
    log_items_inline("user_id", 42); // kötü stil
    log_items("session_id", 99);     // önerilen kullanım

    // Struct üzerinden where kullanımı
    let report = Report {
        title: "Kullanıcı Raporu",
        data: vec!["Baran", "Zeynep", "Ali"],
    };
    report.print();

    // Closure örneği
    let closure = |x| println!("Received value: {}", x);
    call_with_callback(closure);

    // Gerçek API log örneği
    log_request("/api/login", vec!["username", "password"]);
}
