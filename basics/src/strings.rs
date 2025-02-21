pub fn run() {
    // 📌 `&str` → String Slice (Değişmez, sabit boyutlu)
    // Stack'te tutulur ama verisi sabittir. Hızlı ve güvenlidir.
    let name: &str = "Rust"; 
    println!("I am immutable: {}", name);

    // 📌 `String` → Heap String (Dinamik, büyüyebilir)
    // Heap'te tutulur, değiştirilebilir. Kullanıcıdan veri almak için idealdir.
    let mut lang = String::from("Swift"); 
    println!("I can grow: {}", lang);

    // 🔹 `to_string()` ile &str → String dönüşümü
    let another_lang = "Kotlin".to_string();  
    println!("I am now a String: {}", another_lang);

    // 🔹 `push_str()` ile mevcut bir String'e ekleme yapabiliriz.
    lang.push_str(" and Rust"); 
    println!("My new favorite: {}", lang);

    // 🔹 `push()` ile tek karakter ekleyebiliriz.
    lang.push('!'); 
    println!("Now with more excitement: {}", lang);

    // 🔹 `len()` ile kaç byte tuttuğunu öğrenebiliriz.
    let new_lang = lang.replace("Swift", "Go"); 
    println!("Updated list: {}", new_lang);

    // 🔹 Get byte length using `len()`
    println!("'{}' takes up {} bytes", new_lang, new_lang.len());

    // 📌 Unicode desteği: Japonca "こんにちは" (Konnichiwa)
    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{308F}";  
    println!("Japanese greeting: {}", konnichiwa);
}
