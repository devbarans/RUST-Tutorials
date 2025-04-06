// ------------------------------------
// Swift'teki protocol yapısına benzer şekilde,
// Rust'ta da trait kullanarak ortak davranışları soyutlayabiliriz.
// Bu örnek, bir servis (Service) trait'i üzerinden HealthCheck yapısını kontrol eder.
// ------------------------------------

/// Service trait'i, bir servisin temel davranışlarını tanımlar.
/// - activate: servisi aktif hale getirir.
/// - deactivate: servisi pasif hale getirir.
/// - status: servisin mevcut durumunu bir String olarak döner.
trait Service {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn status(&self) -> String; // Sadece okuyucu – &self
}

/// HealthCheck yapısı, bir servisin online olup olmadığını temsil eder.
#[derive(Debug)]
struct HealthCheck {
    is_online: bool,
}

/// Service trait'ini HealthCheck tipi için implement ediyoruz.
/// Bu sayede HealthCheck tipi, Service trait'inde tanımlı olan activate, deactivate ve status fonksiyonlarına sahip olur.
/// Bu işlem, Swift'teki protocol extension ile benzer bir işlevsellik sağlar.
impl Service for HealthCheck {
    // &mut self: Burada nesne üzerinde değişiklik yapıyoruz, bu yüzden mutable referans kullanıyoruz.
    fn activate(&mut self) {
        self.is_online = true; 
        println!("HealthCheck activated"); 
    }

    fn deactivate(&mut self) {
        self.is_online = false; 
        println!("HealthCheck deactivated"); 
    }

    // status fonksiyonu, HealthCheck'in mevcut durumunu döndürür.
    // &self: Burada sadece durumu okuyoruz, nesne üzerinde herhangi bir değişiklik yapmıyoruz.
    fn status(&self) -> String {
        // Eğer HealthCheck çevrimiçi ise, durumu online olarak döndürüyoruz.
        if self.is_online {
            "HealthCheck is online".to_string()
        } else {
            // Eğer çevrimdışı ise, durumu offline olarak döndürüyoruz.
            "HealthCheck is offline".to_string()
        }
    }
}


/// Bu fonksiyon generic olarak herhangi bir `Service` trait'ini implement eden
/// yapılarla çalışabilir. Yani `HealthCheck`, `NetworkService`, `Logger` vs.
/// T: Service bir trait bound'dur.
fn sample<T: Service>(service: &mut T) {
    service.activate();
    println!("Durum: {}", service.status());

    service.deactivate();
    println!("Durum: {}", service.status());
}

// dyn ile dinamin trait nesneleri oluşturabiliriz.
// Bu, trait nesneleri ile çalışırken daha fazla esneklik sağlar.
// Ancak, bu tür nesnelerle çalışırken performans kaybı yaşayabiliriz.
// Bu nedenle, performans kritik durumlarda static trait nesnelerini tercih etmek daha iyidir.
// Örnek olarak, aşağıdaki gibi bir trait nesnesi oluşturabiliriz:
/// `sample_dyn` fonksiyonu, `dyn Service` trait'i kullanarak dinamik dispatch yapar.
/// - Dinamik dispatch kullanıldığı için, hangi fonksiyonun çağrılacağı çalışma zamanında belirlenir.
/// - Bu fonksiyon, herhangi bir `Service` türünü kabul edebilir ve fonksiyonu esnek hale getirir.
fn _sample_dyn(service: &mut dyn Service) {
    service.activate();
    // Burada `service.status()` çağrıldığında, hangi `status` fonksiyonunun çalıştırılacağı
    // çalışma zamanında belirlenecektir.
    println!("Durum: {}", service.status());

    service.deactivate();
    println!("Durum: {}", service.status());
}



/// `run` fonksiyonu, programı başlatır.
/// Burada HealthCheck örneği üzerinden sample fonksiyonunu çağırıyoruz.
/// Okunabilirlik açısından aşağıda tanımladım
pub fn run() {
    let mut service = HealthCheck { is_online: true };
    sample(&mut service);
}


