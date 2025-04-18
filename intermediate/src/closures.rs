/*
Closure'lar — Rust'ta Anonim Fonksiyonlar

Closure'lar (kapanışlar), Rust'ta fonksiyonel programlamayı destekleyen anonim fonksiyonlardır.
Bir closure, tıpkı fonksiyonlar gibi parametre alabilir, bir değer döndürebilir ve ayrıca tanımlandığı çevredeki değişkenlere erişebilir.

Temel Söz Dizimi:
let closure = |parametreler| { işlem };

Özellikleri:
- Tip belirtimi zorunlu değildir; Rust type inference ile çözer.
- Dış değişkenlere erişebilir ve sahipliğini `move` ile alabilir.
- Genellikle fonksiyonlara argüman olarak geçilir veya değişkenlere atanır.
- Heap üzerinde saklanabilir, fonksiyonlara göre daha esnektir.

Function Traits:
Rust'ta closure'lar 3 farklı trait'e göre davranır:

1. `Fn` – read-only erişim: closure içinde dış değişkenler değiştirilemez.
2. `FnMut` – mutable erişim: closure dış değişkenleri değiştirebilir.
3. `FnOnce` – sahiplik alır: dış değişken bir kez kullanılır, sonra closure kapanır.

Ek Bilgi:
- `||` → closure parametrelerini tanımlar.
- `move` → dış değişkenlerin sahipliğini closure'a taşır.
- `where` → generic closure parametrelerinde trait bound tanımlamak için kullanılır.
- `impl Fn`, `impl FnMut`, `impl FnOnce` → closure parametreleri için trait bound'lar.
- Generic isimlendirme kuralları: `F`, `T` gibi tek harfli isimler kullanılır. Closure'lar genellikle `F` ile gösterilir. Tür tipleri ise T 
*/

pub fn run() {
    // Basit closure tanımı (Fn trait)
    let add = |x: i32, y: i32| x + y;
    let result = add(5, 10);
    println!("Closure sonucu (Fn): {}", result);

    // FnOnce örneği - sahiplik alınır (move ile)
    let name = String::from("Baran");
    let greet = move || {
        // name artık greet içinde kullanılabilir, dışarıdan alınmıştır
        println!("Hello, {}!", name);
    };
    greet(); // yalnızca bir kez çağrılabilir (FnOnce)

    // FnMut örneği - dış değişken mut edilebilir
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter(FnMut): {}", counter);
    };
    increment(); // FnMut, değişiklik yapabilir

    // Çoklu iş parçacığı (thread) örneği
    // move ile closure thread’e taşınır
    let message = String::from("Insider thread!");
    let handle = std::thread::spawn(move || {
        println!("{}", message);
    });
    handle.join().unwrap();

    // Closure'ı parametre olarak alma (generic Fn trait bound)
    call_with_closure(|x| println!("Generic closure : {}", x));

    // Closure döndüren fonksiyon
    let multiplier = make_multiplier(3);
    let value = multiplier(5); // 3 * 5 = 15
    println!(" Result (Fn closure return): {}", value);
}

// Trait bound ile closure alan fonksiyon
fn call_with_closure<F>(f: F)
where
    F: Fn(i32),
{
    f(42);
}

//  Closure döndüren fonksiyon örneği
// `impl Fn(i32) -> i32` → i32 alan ve i32 döndüren closure
fn make_multiplier(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}
