// Bu dosya: `src/lib.rs`
// Rust'ta test yazarken genellikle kütüphane (library) modunda çalışırız,
// çünkü testler `lib.rs` veya modüller içinde tanımlanır.

// Basit bir matematik modülü tanımlıyoruz.
pub mod math {
    // Toplama fonksiyonu: İki sayıyı toplar ve sonucu döndürür.
    // `pub` ile fonksiyonu dışarıya açıyoruz ki testlerde kullanılabilsin.
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Çıkarma fonksiyonu: İlk sayıdan ikinciyi çıkarır.
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

// Test modülü: Testler genellikle `#[cfg(test)]` ile işaretlenir.
// Bu, sadece `cargo test` çalıştırıldığında derlenir.
#[cfg(test)]
mod tests {
    // Test modülünde, test edilecek kodları kullanmak için üst modülü içe aktarıyoruz.
    use super::math::*;

    // Bir test fonksiyonu: `#[test]` ile işaretlenir.
    // Test başarılıysa hiçbir şey döndürmez, başarısızsa `panic!` ile hata verir.
    #[test]
    fn test_add() {
        // `assert_eq!` makrosu ile iki değerin eşitliğini kontrol ediyoruz.
        // İlk parametre beklenen değer, ikinci parametre test edilen fonksiyonun sonucu.
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    // Başka bir test fonksiyonu: Çıkarma işlemini test ediyoruz.
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(3, 5), -2);
        assert_eq!(subtract(0, 0), 0);
    }

    // Hata durumu testi: Yanlış bir sonuç bekleyerek testi kontrol edebiliriz.
    #[test]
    fn test_add_failure() {
        // `assert_ne!` ile iki değerin eşit *olmadığını* kontrol ediyoruz.
        assert_ne!(add(2, 2), 5); // 2 + 2 = 5 değil, bu test geçer.
    }

    // Panik testi: Eğer fonksiyonun `panic!` atmasını bekliyorsak,
    // `#[should_panic]` kullanılır. Örnek olarak basit bir hata durumu.
    #[test]
    #[should_panic(expected = "Hata!")]
    fn test_panic() {
        // Örnek bir panik durumu.
        panic!("Hata!");
    }
}

