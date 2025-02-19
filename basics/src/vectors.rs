   
/**
    Vector, aynı türdeki değerleri bir arada tutan bir veri yapısıdır.
    Vector, dinamik boyutlu ve sabit türdür. Yani bir vector'ün eleman türü sabit olmalıdır, ancak eleman sayısı değişebilir.
    Vector, standart kütüphanede tanımlanmıştır. Kullanmak için std::vec::Vec modülünü eklememize gerek yoktur.     Vector, heap'te tutulur. Bu nedenle, vector'ün boyutu çalışma zamanında değiştirilebilir.
    Vector tanımlamak için Vec::new() fonksiyonunu kullanabiliriz.
 */
    /**
    💡 Vektörler (Vec<T>) dinamik dizilerdir.
    🔥 Array'ler sıkıcıdır, sabit boyutludur.
    📦 Rust'taki array'in "serbest ruhlu, genişlemeyi seven" kuzeni gibidir.
    🚀 Vektörler ise esnektir, büyüyebilir, küçülebilir ve heap üzerinde saklanır.
    */

pub fn run() {
    let _vectors = vec![1, 2, 3, 4, 5]; // Vector tanımladık.Türü belirtmedik. Rust tür çıkarımı yaparak türü belirler.
    let vectors: Vec<i16> = vec![1, 2, 3, 4, 5]; // Vector tanımladık. Türünü belirttik.
    println!("Vectors: {vectors:?}"); // :? Tüm elemanları yazdırır.-> Vectors: [1, 2, 3, 4, 5]

    let mut scores: Vec<i32> = vec![10, 20, 30, 40, 50]; // Mutable vector tanımladık.Yani elemanları değiştirebiliriz.
    scores.push(60); // Vector'e yeni bir eleman ekledik.
    scores.push(75); 
    print!("{:?}", scores); // [10, 20, 30, 40, 50, 60]
    // 📦 Vector'ler, heap'te tutulduğu için, elemanlar eklenip çıkarıldıkça bellek yeniden boyutlandırılır.
    // 📦 Bu nedenle, vector'lerin elemanlarına erişmek, eklemek ve çıkarmak, array'lere göre daha maliyetlidir.
    // 📦 Vector'ler, array'lerden daha esnek ve kullanışlıdır, ancak performans açısından biraz daha maliyetlidir
    scores.pop(); // Vector'den son elemanı çıkardık.
    print!("{scores:?}"); // [10, 20, 30, 40, 50,60]
    scores.remove(2); // Vector'den 0. elemanı çıkardık.
    print!("{scores:?}"); // [20, 40, 50, 60]
    scores[0] = 25; // Vector'ün 0. elemanını değiştirdik.

    // unwrap() fonksiyonu, Option türündeki değerlerin içindeki değeri döndürür.
    // Eğer Option türündeki değer None ise unwrap() fonksiyonu programı çöker.
    let last_score = scores.pop().unwrap(); 
    println!("Last Score: {last_score}"); // Last Score: 60

    

    // Vector'ün elemanlarına indeksle erişebiliriz.
    // _ ile başlayan değişkenlerin değerlerini kullanmadığımızı belirtiriz.Buna dikkat edilir
    let _first_score = scores[0]; // scores vector'ünün 0. elemanını aldık.

    // iter() fonksiyonu vector'ün elemanlarını okumak için kullanılır.
    for score in scores.iter(){
     // iter() kullanıldığında, vektörün elemanları üzerinde sadece okuma işlemi yapılır ve vektör değişmez.
     // Eğer kullanılmazsa kullanıldığında, vektörün elemanları üzerinde değişiklik yapılabilir ve vektörün sahipliği döngüye geçer.             * 
    
        println!("Score: {score}"); // Vector'ün elemanlarını yazdırdık.

        
    } // scores vector'ünün sahipliği döngüden çıktığında geri alınır. 

   // new ile bir initilaze edilmiş vector oluşturulur.Constructor gibi düşünebiliriz.
   // İlk başta tipi Vec<T> olarak belirsiz belirtir rust.İlk girdi yapılınca tip belirlenir.
   // Boş bir vector tanımladık.
   // mut anahtar kelimesi ile vector'ü değiştirilebilir yaptık.
    let mut colors_constant = Vec::new(); 
    let mut color_dynamic = Vec::new();
    
    // Vec<&str>
    // Bu satırda "Red" string dilimi (&str) türünde bir eleman olarak eklenir.
    //&str türü, Rust'ta sabit uzunlukta ve değiştirilemez bir string dilimidir.
    colors_constant.push("Red"); // Vector'e yeni bir eleman ekledik.

    // Vec<String>
    // Bu satırda "Green" string türünde bir eleman olarak eklenir.
    // String türü, Rust'ta değiştirilebilir ve dinamik uzunlukta bir string türüdür.
    color_dynamic.push(String::from("Green")); // Vector'e yeni bir eleman ekledik.
    
    // println ile yeni bir satır karakteri eklenir
    // unwrap() fonksiyonu, Option türündeki değerlerin içindeki değeri döndürür.
    println!("Constant String: {}", colors_constant.first().unwrap());

    // print ile yeni bir satır karakteri eklenmez
    print!("Dynamic String: {}", color_dynamic.first().unwrap());
    // 📦 Vector'ler, heap'te tutulduğu için, elemanlar eklenip çıkarıldıkça bellek yeniden boyutlandırılır.

    // collect() fonksiyonu, bir iterator'den bir koleksiyon oluşturmak için kullanılır.
    let codes: Vec<u8> = (10..=20).collect(); // 10'dan 20'ye kadar olan sayıları içeren bir vector oluşturduk.
    println!("{:?}", codes); // [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]

    // Vector'ün belirli bir aralığına erişmek için slicing yapabiliriz.
    let two_codes: Vec<u8> = codes[1..3].to_vec(); // Vector'ün 1. ve 2. elemanlarını alarak yeni bir vector oluşturduk.
    println!("{:?}", two_codes); // [11, 12]

}