use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

/// Thread'ler ile paralel çalışan pozitif sayıları işleme fonksiyonu
/// * `thread_count` - Kullanılacak thread sayısı
fn process_positive_numbers_threaded(numbers: Vec<i32>, thread_count: usize) -> Vec<i32> {
    // Sonuç vektörünü thread'ler arasında güvenli bir şekilde paylaşmak için Arc ve Mutex kullanıyoruz
    let result = Arc::new(Mutex::new(Vec::new()));
    
    // Thread'leri tutacağımız vektör
    let mut handles = vec![];
    
    // Her thread'e düşecek eleman sayısını hesaplıyoruz
    let chunk_size = (numbers.len() + thread_count - 1) / thread_count;
    
    // Vektörü chunk'lara bölüyoruz
    let chunks = numbers.chunks(chunk_size);
    
    // Her chunk için bir thread oluşturuyoruz
    for chunk in chunks {
        // Her thread kendi chunk'ının kopyasını alacak
        let chunk_vec = chunk.to_vec();
        // Arc'ı klonluyoruz ki her thread kendi kopyasına sahip olsun
        let result_clone = Arc::clone(&result);
        
        // Yeni thread oluşturuyoruz
        let handle = thread::spawn(move || {
            // Chunk üzerinde işlemleri yapıyoruz
            let processed = chunk_vec.into_iter()
                .filter(|&x| x > 0)
                .map(|x| x * 2)
                .collect::<Vec<i32>>();
            
            // Mutex'i kilitleyin ve sonuçları ana vektöre ekleyin
            let mut result_vec = result_clone.lock().unwrap();
            result_vec.extend(processed); // İşlenmiş elemanları sonuç vektörüne ekliyoruz
        });
        
        // Thread'i vektöre ekliyoruz
        handles.push(handle);
    }
    
    // Tüm thread'lerin tamamlanmasını bekliyoruz
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Son sonucu Mutex'ten çıkarıp döndürüyoruz
    // try_unwrap ile Arc'ı alıyoruz, çünkü bu işlem başarılı olmalı
    let final_result = Arc::try_unwrap(result)
        .unwrap()
        .into_inner()
        .unwrap();
    
    println!("Process positive numbers with threads: {:?}", final_result);
    
    final_result
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


///Paralel çalışmak daha avantajlıdır. Standart yöntem ile rastgele dizimiz alınmıştı rayon ile sıraya uygun aralarında perfonmans da yakın ama seri şekilde yazılan eski yöntem pek performanslı değil.
fn main() {
    let numbers = vec![1, -2, 3, -4, 5, 6, -7, 8, 9, -10];
    // Normal versiyon
    process_positive_numbers(numbers.clone());
    // Thread'li versiyon - 4 thread kullanarak
    process_positive_numbers_threaded(numbers.clone(), 4);
    // Rayon versiyonu
    rayon_positive_numbers(numbers);
}

