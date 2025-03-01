/// Lifetimes - Rust'ta Referansların Yaşam Süresi
/// Lifetimes, bir referansın ne kadar süreyle geçerli olduğunu belirler. Rust, bellek güvenliği sağlamak için
/// referansların yanlışlıkla geçerliliği sona ermiş bellek bölgelerine erişmesini engeller. Rust'ta her referans
/// bir ömrü (lifetime) vardır ve bu ömür, hangi veri yapısının ne zaman geçerli olduğunu belirler.
/// 
/// Bu örnekte, `Person` adlı bir struct'ı ve onunla birlikte kullanılan lifetimes kavramını ele alacağız.
/// Struct'ımızda `name` değişkeni bir referans olarak tutuluyor, bu nedenle Rust bu referansın ne kadar süreyle
/// geçerli olduğunu bilmek zorundadır. Bu, lifetime'ları anlamak ve yönetmek için kritik bir konudur.

pub fn run() {
    // name değişkeni bir String nesnesidir ve `String::from` ile oluşturulmuştur.
    let name: String = String::from("Baran");

    // Person struct'ı oluşturulurken, name referansı &name olarak struct'a geçer.
    // Person struct'ı lifetime parametresi ('_) ile belirtilmiştir.
    // Bu, name'in geçerlilik süresi, person struct'ının yaşam süresiyle aynı olmalıdır.
    let person: Person<'_> = Person {
        name: &name, // name referansı burada geçer.
        age: 23       // age bir değer olarak geçer, dolayısıyla lifetimes gerektirmez.
    };

    // person objesinin name ve age özelliklerini ekrana yazdırıyoruz.
    println!("Name: {}, Age: {}", person.name, person.age);
}

/// Person struct'ı, lifetime `'a` kullanılarak bir referans tipiyle tanımlanmıştır.
/// Bu struct, `name` referansının, `name` değişkeninin ömrüyle uyumlu olmasını sağlar.
/// `age` ise bir değer olduğu için lifetimes'a ihtiyaç duymaz.
struct Person<'a> {
    name: &'a str,  // 'a lifetime ile işaretlenmiş bir referanstır.
    age: u8         // Yaş bir değerdir, lifetimes gerektirmez.
}

// Lifetimes hakkında daha detaylı bir açıklama ve örnekler:
//
// 1. **Lifetime Annotation - `'a`**
//    - `Person<'a>` gibi bir lifetime parametresi, `Person` struct'ının `name` referansının 
//      geçerliliğini belirtmek için kullanılır. Bu, `Person` struct'ı ile ilişkilendirilen
//      tüm referansların, aynı yaşam süresine sahip olduğunu belirtir.
//
// 2. **Geçici Lifetimes**
//    - Rust'ta, bir fonksiyon parametresine referans verdiğinizde, bu referansın ne kadar süreyle geçerli olduğunu
//      belirtmek için lifetime annotation kullanabilirsiniz.
//      ```rust
//      pub fn greet<'a>(name: &'a str) {
//          println!("Hello, {}", name);
//      }
//      ```
//      Burada `'a`, `name` parametresinin geçerlilik süresini belirtir.
//
// 3. **Karmaşık Lifetimes: Fonksiyonlar Arası İlişkiler**
//    - Lifetimes, birden fazla parametreyle çalışan fonksiyonlarda karmaşıklaşabilir.
//    ```rust
//    pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//        if s1.len() > s2.len() {
//            s1
//        } else {
//            s2
//        }
//    }
//    ```
//    - Burada `longest` fonksiyonu, iki referans alır ve bunların en uzun olanını döndüren bir fonksiyondur.
//      `'a` lifetime'ı, her iki parametrenin de aynı süre boyunca geçerli olduğunu belirtir ve fonksiyonun döndürdüğü
//      referansın bu süreyi takip etmesini sağlar.
//
// 4. **Struct ve Fonksiyon Lifetimes:**
//    - Eğer bir struct, diğer struct'lardan veya fonksiyonlardan gelen referanslarla birlikte çalışıyorsa,
//      her birinin lifetimes'ını belirtmek önemlidir.
//    ```rust
//    struct Book<'a> {
//        title: &'a str,
//        author: &'a str,
//    }
//
//    pub fn print_book<'a>(book: &'a Book<'a>) {
//        println!("Book title: {}, Author: {}", book.title, book.author);
//    }
//    ```
//    - Burada `Book` struct'ı ve `print_book` fonksiyonu, aynı lifetime `'a` parametresini kullanarak uyumlu çalışır.
//
// 5. **Lifetime elden geçirme - Lifetime Boundlar**:
//    - Lifetime'ları belirlemek bazen karmaşık olabilir, özellikle trait'ler ve generic'lerle çalışırken.
//    ```rust
//    pub fn longest_with_lifetime<'a, T>(x: &'a T, y: &'a T) -> &'a T
//    where T: SomeTrait {
//        // some code
//    }
//    ```
//    - Bu örnekte, bir generic tip `T` kullanılırken, her iki parametrenin ve fonksiyonun döndürdüğü referansın
//      aynı lifetime'a sahip olduğunu belirtmek için `'a` lifetime kullanılmıştır.  
// **Özet:**
// Lifetimes, Rust'ta bellek güvenliğini sağlamak için çok önemli bir kavramdır. Referansların geçerlilik süresini
// net bir şekilde tanımlamak, bellek sızıntıları ve erişim hatalarını önler. Fonksiyonlarda ve struct'larda
// lifetimes kullanarak, referansların ne kadar süreyle geçerli olduğunu belirtmek gereklidir. Rust, bu mekanizmayla
