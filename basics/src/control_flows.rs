use rand::rngs::ThreadRng; // `ThreadRng` türünü kullanmak için `rand` modülünden içe aktarıyoruz.
use rand::Rng; // Rastgele sayı üretimi için `Rng` trait'ini içe aktarıyoruz.

/// `run` fonksiyonu rastgele sayı üretme, kontrol yapıları ve döngüler içeren bir demo sunar.
pub fn run() {
    // Rastgele sayı üretmek için RNG nesnesi oluşturuyoruz.
    let mut rng = rand::rng();
    
    // 1 ile 100 arasında rastgele bir tam sayı üretilir.
    let random_number: i32 = rng.random_range(1..=100);
    println!("Random number: {}", random_number);

    // Koşullu ifadeler (if-else)
    if random_number > 50 {
        println!("Random number is greater than 50");
    } else if random_number < 50 {
        println!("Random number is less than 50");
    } else {
        println!("Random number is equal to 50");
    }

    // Sonsuz döngü (`loop`) ve `break` / `continue` kullanımı
    let mut counter = 0;
    loop { 
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 10 {
            break; // Sayaç 10 olduğunda döngüyü kır.
        } else {
            continue; // Geri kalan kodları atlayıp döngünün başına dön.
        }
    }

    // `while` döngüsü kullanımı: Belirtilen koşul sağlandıkça çalışır.
    let mut counter2 = 0;
    while counter2 < 10 {
        println!("Counter2: {}", counter2);
        counter2 += 1;
    }

    // `get_random_numbers` fonksiyonunu çağırarak 10 adet rastgele sayı içeren bir vektör oluşturuyoruz.
    let data = get_random_numbers(rng, 10);
    
    // `enumerate()` kullanarak index ve değeriyle birlikte yazdırıyoruz.
    for (index, value) in data.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}

/// Belirtilen `upper_limit` kadar rastgele sayı üretir ve bir vektör içinde döndürür.
///
/// # Parametreler
/// - `rng`: Rastgele sayı üretimi için `ThreadRng` nesnesi.
/// - `upper_limit`: Kaç adet rastgele sayı üretileceğini belirler.
///
/// # Dönüş Değeri
/// Rastgele üretilmiş `i32` değerlerinden oluşan bir `Vec<i32>`.
fn get_random_numbers(mut rng: ThreadRng, upper_limit: u8) -> Vec<i32> {
    let mut numbers = Vec::new(); // Boş bir vektör oluşturulur.

    // `upper_limit` kadar rastgele sayı üreten döngü.
    for _ in 0..upper_limit {
        let random_number: i32 = rng.random_range(1..=100);
        
        // Eğer sayı zaten vektörde varsa, tekrar etmemesi için `continue` ile döngüyü başa al.
        if numbers.contains(&random_number) {
            continue;
        }
        
        numbers.push(random_number);
    }

    return numbers; // Üretilen rastgele sayıların bulunduğu vektörü döndür.
}
