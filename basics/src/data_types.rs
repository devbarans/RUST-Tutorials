// pub anahtar kelimesi ile fonksiyonu dışarıdan erişilebilir hale getiriyoruz.
// Bu sayede main.rs dosyasından bu fonksiyonu çağırabiliriz.
// Bu dosyada veri tipleri ile ilgili örnekler bulunmaktadır.
// Swiftte ki public anahtar kelimesi ile aynı işlevi görür.

pub fn run(){
    // String değişken tanımladık
    let name: String = String::from("Rust");

    // Ekrana yazdırdık. {} yerine name değişkenini yazdırdık.Süslü paraz ile yazdırma rust +1.56 sürümü sonrası güncel sürümde çalışıyor.
    println!("Hello, {name}");

    // Değişkenin türünü belirtmeden tanımladık. Rust tür çıkarımı yaparak değişkenin türünü belirler.
    // Değişkenin türünü belirtmek zorunda değiliz.
    // Swift gibi dillerce camelCase ama rust da snake_case kullanıyoruz.
    // Hatırlatma: Rust'ta değişkenler varsayılan olarak immutable'dir.
    // i32, signed 32-bit integer türüdür, yani negatif ve pozitif tam sayıları temsil edebilir.
    // Değişkeni kullanmadığımızda _ ekleyerek değişkeni kullanmadığımızı belirtebiliriz.
    let _player_score: i32 = 100;
    // player_score += 1; // Hata verir. Çünkü değişken immutable'dir.
   
    // mutable değişken tanımladık. Değişkenin başına mut ekleyerek değişkeni mutable yaparız. Bu sayede değişkenin değerini değiştirebiliriz.
    // Türü rust i32 olduğunu anlar. Karmaşık projelerde tür belirtmek daha iyi olabilir.
    let mut player_score2 = 99;
    player_score2 += 1 ; // Hata vermez. Çünkü değişken mutable'dir.
    println!("Player score2: {player_score2}");

    // u8, unsigned 8-bit integer türüdür, yani yalnızca pozitif tam sayıları (ve sıfırı) temsil edebilir.
    // 0 ile 255(dahil) arasında değer alabilir.
    let _total_points: u8 = 5 + 3;

    // f32, 32-bit floating-point türüdür, yani ondalık sayıları temsil eder.
    let _pi: f32 = 3.14159;

    // bool, boolean türüdür, yani true veya false değerlerini temsil eder.
    let _is_game_over: bool = false;

    // char, karakter türüdür, yani tek bir karakteri temsil eder.  
    let _first_letter: char = 'R';

    // Tuple, farklı türlerdeki değerleri bir arada tutan bir veri yapısıdır.
    // Tuple, sabit boyutlu ve sabit türdür. Yani bir tuple'ın içindeki eleman sayısı ve türleri sabit olmalıdır
    // Tuple'ın elemanlarına erişmek için indeks kullanılır. İndeksler 0'dan başlar.
    let person: (&str, i32, bool) = ("Baran", 25, true);    
    println!("Name: {}", person.0);  // 0. eleman (isim)
    println!("Age: {}", person.1);   // 1. eleman (yaş)
    println!("Is student: {}", person.2);  // 2. eleman (öğrenci durumu)
    
    
    // Array, aynı türdeki değerleri bir arada tutan bir veri yapısıdır.
    // Array, sabit boyutlu ve sabit türdür. Yani bir array'in eleman sayısı ve türü sabit olmalıdır.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {numbers:?}"); // :? Tüm elemanları yazdırır.-> Numbers: [1, 2, 3, 4, 5]
    
    // hexadecimal, binary ve octal sayılar
    let bg_color : i32 = 0x00FF00; // Hexadecimal
    let gate_flag : i32 = 0b11110000; // Binary
    let dir_permission : i32 = 0o77; // Octal
    println!("Hexadecimal: {bg_color:0x}");
    println!("Binary: {gate_flag:b} / {gate_flag}");
    println!("Octal: {dir_permission:o} / {dir_permission}");

    // Rust'ta const ile sabit tanımlama yapabiliriz.
    println!("Background color: {BACKGROUND_COLOR:0x}");

}   
// const ile sabit tanımlama, değişkenin değeri değiştirilemez. 
// const ile tanımlanan değişkenler büyük harfle yazılır ve alt çizgi ile ayrılır. 
const BACKGROUND_COLOR: i32 = 0x00FF00; // Hexadecimal