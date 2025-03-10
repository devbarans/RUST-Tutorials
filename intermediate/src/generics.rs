// --------------------------------------------------------------
// Rust Generics Advanced Tutorial
// --------------------------------------------------------------
// Bu tutorial, Rust'ın generics (jenerik tipler) mekanizmasını derinlemesine 
// incelemektedir. Aşağıdaki örneklerde, generic fonksiyonlar ve struct'lar
// kullanılarak:
//  - Trait bound'lar (Display, Debug, PartialOrd, Copy) ile tip kısıtlamaları
//    nasıl sağlanır?
//  - Monomorphization: Derleyicinin generic kodu somut tiplere dönüştürerek
//    çalışma zamanı maliyeti olmadan optimize etmesi.
//  - Zero-cost abstractions: Abstraction'ların derleme zamanında giderilmesi
//    sayesinde ek bir maliyet yaratmaması.
//  - Associated types: Generic trait'lerde öğelerin tipi nasıl tanımlanır?
//  - Lifetimes: Referansların ömrünü belirterek bellek güvenliğinin sağlanması.
//  - Where Clause: Trait bound'ları okunabilir şekilde tanımlamak için kullanılır.
//
// Notlar:
//  - "format!" makrosu, belirtilen formatta bir String oluşturur; direkt ekrana 
//    yazdırmaz. Ekrana yazdırmak için "println!" kullanılmalıdır.
//  - #[derive(Debug)] attribute'u, ilgili struct'un Debug trait'ini otomatik 
//    olarak implemente etmesini sağlar.
// --------------------------------------------------------------

use std::fmt::{Debug, Display};

// --- 1. Generic Function: user_input ---
// Bu fonksiyon, parametre olarak aldığı herhangi bir tipteki veriyi ekrana yazdırır.
// "T: Display" constraint'i, sadece ekrana yazdırılabilir tiplerin kabul edilmesini sağlar.
// Bu trait bound, Rust'ın statik tip güvenliği ve zero-cost abstraction prensiplerini ortaya koyar.
fn user_input<T: Display>(input: T) {
    println!("User input: {}", input);
}

// --- 2. Generic Struct: Point ---
// Point yapısı, iki boyutlu bir nokta olarak tanımlanır ve x ile y koordinatları için
// generic tip T kullanılır. Böylece, Point hem tamsayı hem de kayan nokta gibi farklı tiplerde
// koordinatları tutabilir.
// Derleyici, bu generic yapıyı somut tipler (örneğin Point<i32> ve Point<f64>) için monomorphization
// işlemi ile ayrı ayrı kod üreterek optimize eder.
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> Point<T> {
    // new: Generic constructor. Self, burada Point<T> anlamına gelir.
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    // info: Noktanın koordinatlarını Debug trait'i kullanarak biçimler.
    // "format!" makrosu, runtime'da verilen formatta String oluşturur.
    fn info(&self) -> String {
        format!("x: {:?}, y: {:?}", self.x, self.y)
    }
}

// --- 3. Generic Function: swap_values ---
// Bu fonksiyon, iki değerin yerlerini değiştirir.
// T tipi için herhangi bir trait bound gerekmez; sadece değerlerin yer değiştirme işlemi yapılır.
// Bu, parametrik polimorfizmin temel örneklerinden biridir.
fn swap_values<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

// --- 4. Generic Struct with Derived Trait: Pair ---
// Pair, iki öğeyi tutan basit bir generic yapıdır.
// #[derive(Debug)] sayesinde, Pair struct'ı otomatik olarak Debug trait'ini elde eder.
// Bu, hata ayıklama ve loglama işlemlerinde nesnenin içeriğini kolayca görüntülemeyi sağlar.
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

// --- 5. Generic Function with Multiple Trait Bounds: max ---
// Bu fonksiyon, iki değeri karşılaştırarak en büyük olanı döndürür.
// Burada T, hem PartialOrd (karşılaştırma) hem de Copy (değer kopyalama) trait'lerini implemente etmelidir.
// Bu örnek, generic fonksiyonlarda birden fazla trait bound kullanımının esnekliğini gösterir.
fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

// --- 6. Generic Function with Lifetimes and Where Clause: find_largest ---
// Bu fonksiyon, bir dilim (slice) içerisindeki en büyük öğeyi bulur.
// Generic tip T, PartialOrd trait'ini implemente etmelidir ki öğeler karşılaştırılabilsin.
// Fonksiyon dönüş değeri, giriş diliminin yaşam süresi (lifetime) ile aynıdır.
// 'where' clause, trait bound'ların okunabilirliğini artırır.
fn find_largest<'a, T>(slice: &'a [T]) -> &'a T 
where 
    T: PartialOrd,
{
    let mut largest = &slice[0];
    for item in slice.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// --- 7. Generic Function with Where Clause: print_pair ---
// Bu fonksiyon, iki öğeyi alır ve ekrana yazdırır.
// 'where' clause kullanılarak, T ve U tiplerinin Display trait'ini implemente etmeleri gerektiği belirtilir.
// Böylece, her iki öğe de "println!" ile düzgün biçimde yazdırılabilir.
fn print_pair<T, U>(a: T, b: U) 
where 
    T: Display,
    U: Display,
{
    println!("Pair: {} and {}", a, b);
}

// --- Ana Çalıştırma Fonksiyonu: run ---
// Bu fonksiyon, yukarıdaki tüm örnekleri çalıştırarak generics kullanımını,
// trait bound'ların (Display, Debug, PartialOrd, Copy) nasıl uygulandığını, 
// monomorphization sürecini, associated types, lifetimes ve where clause gibi 
// ileri seviye kavramları göstermektedir.
// Tek bir çağrıda, her örneğin çıktısı ile generics'in sunduğu esneklik ve 
// performans avantajları ortaya konulmaktadır.
pub fn run() {
    println!("Generics in Rust - Advanced Tutorial");
    println!("----------------------------------------");

    // 1. Generic function örneği: user_input
    let number: i32 = 10;
    user_input(number);
    let message: String = String::from("Hello, World!");
    user_input(message);

    println!("----------------------------------------");

    // 2. Generic struct örneği: Point
    let point_int = Point::new(3, 4);
    println!("Point (int): {}", point_int.info());
    let point_float = Point::new(3.5, 4.2);
    println!("Point (float): {}", point_float.info());

    println!("----------------------------------------");

    // 3. Generic function örneği: swap_values
    let (a, b) = swap_values(5, 10);
    println!("Swapped values: a = {}, b = {}", a, b);

    println!("----------------------------------------");

    // 4. Generic struct örneği: Pair
    let pair = Pair { first: "one", second: "two" };
    println!("Pair Debug: {:?}", pair);

    println!("----------------------------------------");

    // 5. Generic function örneği: max
    let max_value = max(3, 7);
    println!("Max value: {}", max_value);

    println!("----------------------------------------");

    // 6. Generic function with lifetimes and where clause: find_largest
    let numbers = vec![3, 7, 2, 9, 5];
    let largest_number = find_largest(&numbers);
    println!("Largest number in {:?} is {}", numbers, largest_number);

    println!("----------------------------------------");

    // 7. Generic function with where clause: print_pair
    print_pair("Rust", 2021);
}
