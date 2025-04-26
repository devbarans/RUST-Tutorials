use std::cell::RefCell;
use std::rc::Rc;

// Kullanıcı (User) yapısı: Her kullanıcı bir isim, yaş ve arkadaş listesine sahip
// Arkadaş listesi, diğer kullanıcıları işaret eden Rc<RefCell<User>> vektörü
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    friends: Vec<Rc<RefCell<User>>>,
}

impl User {
    // Yeni bir kullanıcı oluşturur ve Rc<RefCell<User>> döner
    // Neden Rc? Birden fazla yerde aynı kullanıcıya referans vermek için (shared ownership)
    // Neden RefCell? Runtime'da mutable/immutable borrow kontrolü için (interior mutability)
    fn new(name: String, age: u8) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(User {
            name,
            age,
            friends: Vec::new(),
        }))
    }

    // Bir kullanıcıya arkadaş ekler
    // Parametreler neden Rc<RefCell<User>>? Çünkü kullanıcılar heap'te ve shared ownership ile yönetiliyor
    fn add_friend(parent: &Rc<RefCell<User>>, friend: Rc<RefCell<User>>) {
        // borrow_mut(): Mutable bir referans alır, friends vektörüne ekleme yapabilmek için
        // Neden borrow_mut? Çünkü friends vektörünü değiştireceğiz (mutable access)
        parent.borrow_mut().friends.push(friend);
    }

    // Kullanıcının arkadaşlarının isimlerini listeler
    // Neden &self yerine Rc<RefCell<Self>>? Çünkü kullanıcılar Rc ile sarılı
    fn list_friends(user: &Rc<RefCell<User>>) -> Vec<String> {
        // borrow(): Immutable bir referans alır, sadece okuma için
        // Neden borrow? Çünkü sadece friends vektörünü okuyacağız (immutable access)
        let user_ref = user.borrow();
        // iter(): Vektörün elemanları üzerinde döner, ownership transfer etmez
        // Neden iter? Çünkü friends vektörünün elemanlarını sırayla gezmek istiyoruz
        user_ref
            .friends
            .iter()
            .map(|friend| friend.borrow().name.clone()) // clone: String'in bir kopyasını alır
            // Neden clone? String ownership gerektirir, borrow'dan kopya oluşturur
            .collect()
    }
}

// Rc, RefCell, borrow, iter ve clone'un kullanımını gösteren ana fonksiyon
pub fn run() {
    // 1. Kullanıcıları oluşturuyoruz
    // Rc::new ve RefCell::new ile heap'te kullanıcıları oluşturuyoruz
    let user1 = User::new(String::from("Alice"), 30);
    let user2 = User::new(String::from("Bob"), 25);
    let user3 = User::new(String::from("Charlie"), 28);

    // 2. Arkadaş ekliyoruz
    // user1'e user2 ve user3'ü ekliyoruz
    // add_friend, Rc<RefCell<User>> parametreleri alır
    User::add_friend(&user1, user2.clone()); // clone: Rc'nin referans sayısını artırır
    User::add_friend(&user1, user3.clone());
    // Neden clone? Rc bir pointer, clone ile referans sayısını artırarak shared ownership sağlıyoruz
    // Rc olmadan aynı veriye birden fazla sahip olamayız (Rust'ın ownership kuralları)

    // 3. user1'in arkadaşlarını listeliyoruz
    // list_friends, friends vektörünü iter ile gezer ve isimleri toplar
    let friends = User::list_friends(&user1);
    println!("{}'s friends: {:?}", user1.borrow().name, friends);

    // 4. Örnek: Aynı kullanıcıyı başka bir yerde kullanmak
    // user2'yi başka bir bağlamda kullanıyoruz (örneğin başka bir arkadaş listesine eklemek)
    let user4 = User::new(String::from("Dave"), 27);
    User::add_friend(&user4, user2.clone()); // user2 hâlâ hayatta, çünkü Rc ile paylaşılıyor
    let user4_friends = User::list_friends(&user4);
    println!("{}'s friends: {:?}", user4.borrow().name, user4_friends);

    // Ekstra demonstrasyonlar
    println!("\nRc Demonstration:");
    demonstrate_rc();

    println!("\nRefCell Demonstration:");
    demonstrate_refcell();
}

// Rc kullanımına basit bir örnek
fn demonstrate_rc() {
    // Rc: Reference Counting, birden fazla sahip (owner) olmasını sağlar
    // Heap'te bir veri oluşturuyoruz ve birden fazla pointer ile işaret ediyoruz
    let data = Rc::new(String::from("Shared data"));
    let data2 = data.clone(); // Yeni bir Rc pointer oluşturur, veri kopyalanmaz
    let data3 = data2.clone();

    // Hepsi aynı heap verisini işaret eder
    println!("data: {:?}", data);
    println!("data2: {:?}", data2);
    println!("data3: {:?}", data3);
    // Rc, veri drop edilene kadar referans sayısını takip eder
    // Son Rc drop edildiğinde veri serbest bırakılır
}

// RefCell kullanımına basit bir örnek
fn demonstrate_refcell() {
    // RefCell: Runtime'da borrow kurallarını uygular, interior mutability sağlar
    let value = RefCell::new(42);

    // borrow: Immutable erişim
    println!("Initial value: {}", *value.borrow());

    // * ile dereference ediyoruz
    // dereference: RefCell içindeki değere erişim sağlar
    // borrow_mut: Mutable erişim
    *value.borrow_mut() += 1;
    println!("Updated value: {}", *value.borrow());

    // RefCell, compile-time yerine runtime'da borrow hatalarını yakalar
    // Örneğin, aynı anda iki mutable borrow alırsak panic olur
    
}
