   
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

    

    // Vector'ün elemanlarına indeksle erişebiliriz.
    // _ ile başlayan değişkenlerin değerlerini kullanmadığımızı belirtiriz.Buna dikkat edilir
    let _first_score = scores[0]; // scores vector'ünün 0. elemanını aldık.



}