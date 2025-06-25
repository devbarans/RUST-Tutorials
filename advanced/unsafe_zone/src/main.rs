
/*
unsafe, Rust'ta derleyicinin normalde garanti edemediği, potansiyel olarak tehlikeli işlemleri (ham pointer, FFI, static mut, vs.) yapabilmek için kullanılan bir anahtar kelimedir. 
Güvenlik garantilerinin bir kısmı programcıya bırakılır.
static mut değişkenlere doğrudan erişmek Rust'ta tehlikelidir ve uyarı alırsınız. Çünkü bu değişkenler globaldir ve birden fazla thread aynı anda erişirse veri yarışına (data race) sebep olabilir.
Bu yüzden Rust 2024 ile birlikte static mut kullanımı önerilmez ve mümkünse kaçınılmalıdır.
*/
fn main() {
    // 1. Raw pointer oluşturma - Raw pointerlar null olabilir ve geçerli bir bellek adresini 
    // göstermeleri garanti değildir
    let mut number = 5;
    let raw_pointer = &mut number as *mut i32;

    // 2. Unsafe blok - Raw pointerlara erişmek için gereklidir
    // Normal Rust'ta yapamayacağımız tehlikeli işlemleri burada yapabiliriz
    unsafe {
        *raw_pointer = 10;
        println!("Raw pointer value: {}", *raw_pointer);
    }

    // 3. Unsafe fonksiyon tanımlama - Bellek güvenliğini garanti edemeyen fonksiyonlar
    // Bu fonksiyonlar tehlikeli işlemler içerebilir
    unsafe fn dangerous_operation() {
        println!("This functions marked as unsafe!");
    }

    // 4. Unsafe fonksiyon çağrısı - Sadece unsafe blok içinde çağrılabilir
    unsafe {
        dangerous_operation();
    }

    // 5. Raw pointer ile bellek işlemleri - Pointer aritmetiği örneği
    // Bellek adreslerini doğrudan manipüle edebiliriz
    let mut array = vec![1, 2, 3, 4, 5];
    let ptr = array.as_mut_ptr();
    
    unsafe {
        // Pointer aritmetiği - Bellek adresleri üzerinde gezinme
        println!("First: {}", *ptr);
        println!("Second: {}", *ptr.add(1));
    }
}

// 6. Unsafe trait tanımlama - Derleyicinin güvenliğini kontrol edemediği durumlar için
unsafe trait DangerousTrait {
    fn dangerous_method(&self);
}

// 7. Unsafe trait implementasyonu - Unsafe trait'leri implement ederken unsafe olarak işaretlenmeli
struct UnsafeStruct;

unsafe impl DangerousTrait for UnsafeStruct {
    fn dangerous_method(&self) {
        println!("DangerousTrait!");
    }
}
