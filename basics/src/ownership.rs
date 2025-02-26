pub fn run() {
    println!("Hello, Ownership and Move!");

    // Değişkenlerin yaşam süresi tanımlandığı scope ile sınırlıdır.
    // Bir değişkenin ömrü, bulunduğu kapsam (scope) sona erdiğinde biter.
    {
        let name = String::from("Baran"); // Heap üzerinde tahsis edilir.
        println!("Name: {}", name);
    }
    // `name` değişkeni scope dışında artık erişilemez, çünkü ownership düşmüştür.

    ownership_and_move();

    let title = String::from("Rust Programming");
    consume_title(title); // `title` değişkeninin sahipliği fonksiyona geçer ve burada drop edilir.

    let title1 = String::from("Swift Programming");
    consume_title(title1.clone()); // Clone ile deep copy oluşturulur, böylece orijinal değişken korunur.
    println!("title1: {}", title1); // `title1` hala geçerlidir.

    let title2 = String::from("Kotlin Programming");
    borrow_title(&title2); // `title2` değişkeni yalnızca referans ile ödünç verilir, ownership değişmez.

    let mut title3 = String::from("Java Programming");
    borrow_mutable_title(&mut title3); // Mutable referans ile değişiklik yapılabilir, ancak tek referans olmalıdır.
    
    mutable_reference_restriction();
    copy_vs_clone();
    stack_vs_heap();
}

fn ownership_and_move() {
    // Ownership sistemi, Rust'ta bellek yönetimini otomatik ve güvenli hale getirir.
    // Sahiplik kuralları gereği, bir değişkenin sahipliği başka bir değişkene aktarıldığında,
    // ilk değişkenin referansı geçersiz hale gelir ve kullanılamaz.
    let s1 = String::from("hello");
    let s2 = s1; // `s1` artık kullanılamaz çünkü sahiplik `s2`'ye geçti.
    println!("s2: {}", s2);

    // Eğer bir nesnenin bağımsız bir kopyasını oluşturmak istersek `clone()` metodunu kullanmalıyız.
    let s3 = String::from("Hey");
    let s4 = s3.clone(); // Heap verisi tamamen kopyalanır, her iki değişken bağımsızdır.
    println!("s3: {}, s4: {}", s3, s4);
}

// Sahipliği devralan fonksiyon, aldığı değerin kontrolünü tamamen üstlenir.
// `title` fonksiyon bitiminde drop edilir ve bellekten temizlenir.
fn consume_title(title: String) {
    println!("Title: {}", title);
}

// Referans ile ödünç verme işlemi, sahipliği değiştirmez, sadece okunur erişim sağlar.
fn borrow_title(title: &String) {
    println!("Borrowed Title: {}", title);
}

// Mutable referans ile değişkenin içeriği değiştirilebilir, ancak aynı anda yalnızca tek bir mutable referans olabilir.
fn borrow_mutable_title(title: &mut String) {
    title.push_str(" - Updated"); // Mutable referans kullanarak değişkenin içeriği değiştirilir.
    println!("Mutable Borrowed Title: {}", title);
}

fn mutable_reference_restriction() {
    // Rust'ta aynı anda birden fazla mutable referans oluşturulamaz.
    let mut s = String::from("Only one mutable reference");
    let r1 = &mut s; // İlk mutable referans oluşturulur.
    println!("{}", r1);
    // let r2 = &mut s; // Hata: Aynı anda ikinci bir mutable referans oluşturulamaz.
}

fn copy_vs_clone() {
    // Stack üzerinde saklanan veri türleri `Copy` trait'ine sahiptir,
    // ve doğrudan kopyalanabilir, ownership aktarımı olmaz.
    let x: i32 = 10;
    let y = x; // `Copy` trait sayesinde x hala kullanılabilir.
    println!("x: {}, y: {}", x, y);

    // Heap üzerinde saklanan veri türleri `Clone` trait'ini kullanarak
    // derin kopyalama yapabilir.
    let s1 = String::from("Hello, Rust");
    let s2 = s1.clone(); // Heap'teki içerik tamamen yeni bir yere kopyalanır.
    println!("s1: {}, s2: {}", s1, s2);
}

fn stack_vs_heap() {
    // Stack ve Heap farkı:
    // Stack, sabit boyutlu ve hızlı erişilebilir veriler için kullanılır.
    // Heap ise dinamik boyutlu verileri saklamak için kullanılır ancak yönetim maliyetlidir.
    
    let stack_var: i32 = 42; // Stack üzerinde saklanır, LIFO prensibi ile yönetilir.
    println!("Stack Variable: {}", stack_var);
    
    let heap_var = String::from("Heap Allocation"); // Heap üzerinde tahsis edilir, yavaş ama esnektir.
    println!("Heap Variable: {}", heap_var);
    
    let heap_clone = heap_var.clone(); // Heap verisi kopyalanır, ekstra bellek tahsisi gerektirir.
    println!("Heap Clone: {}", heap_clone);
}
