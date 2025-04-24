// Rust'ta Box<T> Akıllı İşaretçisi ve Bağlı Liste Örneği
// =================================================================================
// Bu dosya, Rust programlama dilinde `Box<T>` akıllı işaretçisinin kullanımını
// açıklamak için hazırlanmıştır. `Box<T>`, heap üzerinde veri tutmak ve recursive
// yapılar oluşturmak için idealdir. Bu örnekte, bir bağlı liste (linked list)
// yapısı oluşturarak `Box<T>`'ın nasıl çalıştığını adım adım gösteriyoruz.
//
// Kodun yapısı:
// 1. `Node` struct'ının tanımı
// 2. Bağlı liste oluşturma
// 3. Bağlı listeyi yazdırma fonksiyonu
// 4. `Box<T>`'ın temel kullanımı (ek örnek)
// 5. Ana fonksiyon (`main`) ve çalışma akışı

/// Bağlı liste için Node (düğüm) yapısı
/// - `value`: Node'un tuttuğu veri (i32 tipi)
/// - `next`: Bir sonraki Node'a işaret eden `Option<Box<Node>>`
///   - `Some(Box<Node>)`: Bir sonraki node'a işaret eder
///   - `None`: Listenin sonu
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

/// Bağlı listeyi baştan sona gezerek her node'un değerini yazdıran fonksiyon
/// - `node`: Listenin başındaki node'un referansı (`&Node`)
fn print_list(node: &Node) {
    let mut current = node; // Başlangıç node'unu referans olarak al
    loop {
        print!("{} -> ", current.value); // Mevcut node'un değerini yazdır
        match &current.next {
            Some(next_node) => current = next_node, // Bir sonraki node'a geç
            None => {
                println!("None"); // Liste bitti, None yazdır
                break; // Döngüden çık
            }
        }
    }
}

/// Ana fonksiyon: Bağlı liste oluşturur ve yazdırır
pub fn run() {
    // Bağlı liste oluştur: 1 -> 2 -> 3
    // node3: Listenin son elemanı, next = None
    let node3 = Node {
        value: 3,
        next: None,
    };

    // node2: 2 değerini tutar, next ile node3'e işaret eder
    let node2 = Node {
        value: 2,
        next: Some(Box::new(node3)), // node3'ü heap'e koy ve işaret et
    };

    // node1: 1 değerini tutar, next ile node2'ye işaret eder
    let node1 = Node {
        value: 1,
        next: Some(Box::new(node2)), // node2'yi heap'e koy ve işaret et
    };

    // Listeyi yazdır
    println!("Bağlı Liste:");
    print_list(&node1); // Çıktı: 1 -> 2 -> 3 -> None
}

// ---------------------------------------------------------------------------------
// EK ÖRNEK: Box<T>'ın Temel Kullanımı
// Aşağıdaki fonksiyon, `Box<T>`'ın heap üzerinde basit bir değer tutma yeteneğini
// ve mutable Box ile değer değiştirme işlemini gösterir.
// ---------------------------------------------------------------------------------

/// Box<T>'ın temel kullanımını gösteren yardımcı fonksiyon
fn simple_box_example() {
    // Box ile heap'te bir tamsayı tutma
    let x = Box::new(42); // 42, heap'te saklanır
    println!("Box içindeki değer: {}", *x); // *x ile değere eriş (dereference)
    
    // Mutable Box ile değer değiştirme
    let mut y = Box::new(10); // Mutable Box
    *y = 20;                  // Değeri 20 yap
    println!("Mutable Box değeri: {}", *y); // Çıktı: 20
}

// Not: Bu fonksiyon main'de çağrılmıyor, ama `Box<T>`'ın temel mantığını anlamak içindir