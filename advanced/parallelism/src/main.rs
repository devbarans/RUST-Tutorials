use rayon::prelude::*;


/// Paralel çalıştırmak sıralı çalıştırmaya göre daha perfomanslıdır.
fn main() {
    process_positive_numbers(vec![1, -2, 3, -4, 5, 6, -7, 8, 9, -10]);
    rayon_positive_numbers(vec![1, -2, 3, -4, 5, 6, -7, 8, 9, -10]);

}

/// Pozitif sayıları filtreler, her birini 2 ile çarpar ve yeni bir vektör döndürür.
fn process_positive_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let processed_numbers = numbers
        .into_iter() // Vektörün elemanlarının sahipliğini alır, böylece orijinal vektör kullanılamaz.
        .filter(|&x| x > 0) // Pozitif olan elemanları filtreler. x ile referans çözülür
        .map(|x| x * 2) // Her elemanı 2 ile çarpar.
        .collect(); // İşlenmiş elemanları yeni bir vektör olarak toplar.

    println!("Process positive numbers: {:?}", processed_numbers);

    // İşlenmiş vektörü döndürüyoruz.
    // return kullanabilirsin ama Rust'ta son ifade otomatik olarak döndürülür.
     processed_numbers
}


/// Rayon kütüphanesini kullanarak paralel işleme yapar.
/// Rayon, paralel iteratörler sağlayarak işlemleri daha hızlı hale getirir.
fn rayon_positive_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let processed_numbers: Vec<i32> = numbers
        .par_iter() // Paralel iteratör kullanarak elemanları işler.
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect(); 

    println!("Process positive numbers with Rayon: {:?}", processed_numbers);

    return processed_numbers
}


