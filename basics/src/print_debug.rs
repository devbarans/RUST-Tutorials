
/**
  * 💡 Rust'ta println! ile yazdırma yaparken formatlama kurallarına dikkat etmek gerekir.
  * 🔥 Debug formatı (`{:?}`) kullanarak struct, enum ve vektör gibi veri tiplerini yazdırabiliriz.
 */
// pub yaparak bu fonksiyonu dışarıdan erişilebilir hale getiriyoruz :))
// Bu sayede main.rs dosyasından bu fonksiyonu çağırabiliriz.
//Eğer sadece bu dosya çalıştırılacaksa pub anahtar kelimesine gerek yoktur.
//Eğer bu dosya burada çalışacaksa run değiştir main yapılmalıdır.Bu dosya çalıştırılacaksa main fonksiyonu olmalıdır.
pub fn run() {
    let name = "Rust";
    let numbers = vec![1, 2, 3];
           
    // Debug formatıyla yazdırmak için println!("{:?}", variable_name) kullanılır. Rust uygun bir şekilde yazdırır.


    // ✅ Normal ekrana yazdırma.Ama debug formatı öneririm çünkü rust daha güzel yazdırır.
    println!("{}", name); 

    // ✅ Debug formatında yazdırma. Struct ve enum türlerini yazdırmak için kullanışlıdır.
    println!("{:?}", numbers);  // 📌 [1, 2, 3]
    
    // ✅ Daha okunaklı debug formatı (JSON benzeri çıktı)
    println!("{:#?}", numbers);
    
    // ❌ Hatalı kullanımlar: Format belirtilmediği için hata verir.
    // println!("{:?}", name);  // ⚠️ Stringler için debug formatı desteklenmez!
    // println!("{scores}");    // ⚠️ Format belirtilmediği için hata!
    // println!(42);            // ⚠️ Sayı doğrudan yazılamaz! String olmalı!
    // println!("{}");          // ⚠️ Değişken belirtilmedi! Hata!
    // println!("{:?}");        // ⚠️ Değişken belirtilmedi! Hata!

}

