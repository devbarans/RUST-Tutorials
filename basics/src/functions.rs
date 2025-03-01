// Rust'ta fonksiyonlar varsayılan olarak `private`'tır.
// Eğer fonksiyonu başka modüllerden çağırmak istiyorsan `pub` anahtar kelimesini eklemelisin.

pub fn run() {
    // Temel fonksiyon çağrıları
    say_hello("Baran");

    let result = sum(5, 10);
    println!("Sum: {}", result);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // `get_odds()` ile fonksiyonel programlama yaklaşımıyla tek sayıları alma
    // & ile referans alıyoruz. Çünkü fonksiyonun içinde değişiklik yapmıyoruz.
    let odds = get_odds(&numbers);
    println!("Odd numbers (functional): {:?}", odds);

    // `get_odds_loop()` ile döngü kullanarak tek sayıları alma
    let odds_loop = get_odds_loop(&numbers);
    println!("Odd numbers (loop): {:?}", odds_loop);

    // `filter_odds_in_place()` ile mevcut vektörü değiştirme
    let mut numbers_in_place = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    filter_odds_in_place(&mut numbers_in_place);
    println!("Filtered in place (only odds remain): {:?}", numbers_in_place);

    // alternative main function
    _main_alternative();

    // Tuple kullanarak birden fazla değer döndüren fonksiyon
    // Fonksiyonun dönüş değerini bir tuple olarak alabiliriz.
    // Tuple'ın içindeki değerleri ayrı ayrı almak için `let (x, y) = ...` şeklinde kullanabiliriz.
    let (x, y) = move_position(10, 20, 5);
    println!("x: {}, y: {}", x, y);
}

// **Parametre alan ve ekrana çıktı veren bir fonksiyon**
fn say_hello(name: &str) {
    println!("Hello, {name}!");
}

// **Toplama işlemi yapan ve sonucu döndüren fonksiyon**
fn sum(a: i32, b: i32) -> i32 {
    a + b // `return` kullanmadan sonucu döndürebiliriz.
}

// **YÖNTEM 1: FONKSİYONEL PROGRAMLAMA YAKLAŞIMI**
// Bir diziden tek sayıları filtreleyip yeni bir vektör döndüren fonksiyon
fn get_odds(numbers: &[i32]) -> Vec<i32> {
    // `[i32]` türünden bir slice alır, `Vec<i32>` döndürür.

    numbers
        .iter()  // Slice (referans) üzerinde bir iterator başlatır.
        .filter(|&n| n % 2 != 0) // Sadece tek sayıları seçer.
        .copied() // `&i32` yerine doğrudan `i32` değerleri döndürür.
        .collect() // Sonuçları `Vec<i32>` olarak toplar.
}

// **YÖNTEM 2: DÖNGÜ KULLANARAK**
// Bir diziden tek sayıları filtreleyip yeni bir vektör döndüren **döngü tabanlı** fonksiyon
fn get_odds_loop(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new(); // Yeni bir vektör oluştur

    for number in numbers { // Eğer `iter()` kullanılsaydı iter ile referanslar alınırdı.
        if number % 2 != 0 { 
            result.push(*number); // *number ile referansı çözüp değeri alıyoruz.
            // eğer in numbers.iter() kullanılsaydı ve if &number % 2 != 0 şeklinde yazılsaydı. result.push(number) şeklinde yazılmalıydı.
        }
    }

    result
}

// **YÖNTEM 3: MUTABLE SLICE KULLANARAK** 
// Bir slice üzerinde değişiklik yaparak çift sayıları kaldırıp yalnızca tek sayıları bırakan fonksiyon.
fn filter_odds_in_place(numbers: &mut Vec<i32>) {
    // `retain()` metodu, verilen koşulu sağlamayan elemanları vektörden kaldırır.
    numbers.retain(|n| *n % 2 != 0);
}

// **ALTERNATİF MAIN FONKSİYONU (Değişkenleri farklı kullanmak için)** 
fn _main_alternative() {
    // `Vec<i32>` (dinamik dizi) oluşturuyoruz.
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //  `filter_odds_in_place` fonksiyonunu çağırıyoruz ve `numbers` değişkenini `&mut` referans olarak geçiyoruz.
    filter_odds_in_place(&mut numbers); 

    //  İşlem tamamlandıktan sonra, vektörün güncellenmiş halini ekrana yazdırıyoruz.
    println!("Filtered in place (only odds remain): {:?}", numbers);
}

// Tuple kullanarak birden fazla değer döndüren fonksiyon
fn move_position(mut x: i32, mut y: i32, acceleration: i32) -> (i32, i32) {
    x+= acceleration;
    y+= acceleration;
    return (x, y)
}
