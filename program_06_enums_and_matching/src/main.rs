// === 1. Mendefinisikan Enum ===
// Enum ini mendefinisikan sebuah Pesan yang bisa menjadi salah satu dari empat varian.
// Perhatikan bahwa varian bisa saja tidak membawa data, atau membawa data dengan tipe yang berbeda-beda.
enum Pesan {
    Keluar, // Varian tanpa data
    Tulis(String), // Varian dengan data String
    UbahWarna(i32, i32, i32), // Varian dengan tiga data i32 (misalnya, RGB)
    Pindah { x: i32, y: i32 }, // Varian dengan struct anonim
}

// Sama seperti struct, kita bisa mendefinisikan method pada enum.
impl Pesan {
    fn panggil(&self) {
        // Kita menggunakan `match` untuk menjalankan kode yang berbeda untuk setiap varian.
        println!("Memproses pesan...");
        match self {
            Pesan::Keluar => {
                println!("Varian 'Keluar': Tidak ada data.");
            }
            Pesan::Tulis(teks) => {
                println!("Varian 'Tulis': \"{}\"", teks);
            }
            Pesan::UbahWarna(r, g, b) => {
                println!("Varian 'UbahWarna': Merah={}, Hijau={}, Biru={}", r, g, b);
            }
            Pesan::Pindah { x, y } => {
                println!("Varian 'Pindah': Koordinat baru x={}, y={}", x, y);
            }
        }
    }
}

// === 2. Enum `Option<T>` dan Penanganan Nilai Kosong ===
// Di banyak bahasa lain, ada konsep `null` atau `nil` yang berarti "tidak ada nilai".
// Ini sering menyebabkan bug (null pointer exceptions).
// Rust tidak memiliki `null`. Sebagai gantinya, Rust memiliki enum `Option<T>`.
//
// enum Option<T> {
//     Some(T), // Ada nilai dengan tipe T
//     None,    // Tidak ada nilai
// }
//
// Compiler memaksa kita untuk menangani kasus `None`, sehingga kita tidak akan pernah lupa.

// Fungsi ini mungkin mengembalikan nilai, mungkin juga tidak.
fn cari_item_berikutnya(s: &[i32]) -> Option<i32> {
    if s.is_empty() {
        None // Tidak ada item, jadi kembalikan None.
    } else {
        Some(s[0] + 1) // Ada item, kembalikan Some yang berisi nilainya.
    }
}


fn main() {
    println!("--- Enum dan Match ---");
    let pesan1 = Pesan::Tulis(String::from("Halo Rust!"));
    let pesan2 = Pesan::Pindah { x: 10, y: 20 };
    let pesan3 = Pesan::UbahWarna(255, 0, 0);
    let pesan4 = Pesan::Keluar;

    pesan1.panggil();
    pesan2.panggil();
    pesan3.panggil();
    pesan4.panggil();

    println!("\n--- Enum Option<T> ---");
    let list_angka = vec![5, 10, 15];
    let item_berikutnya = cari_item_berikutnya(&list_angka);

    // Kita harus menangani kedua kemungkinan dari Option: Some dan None.
    match item_berikutnya {
        Some(angka) => println!("Item berikutnya adalah: {}", angka),
        None => println!("Tidak ada item berikutnya."),
    }

    let list_kosong = vec![];
    let item_kosong = cari_item_berikutnya(&list_kosong);
    match item_kosong {
        Some(angka) => println!("Item berikutnya adalah: {}", angka),
        None => println!("Tidak ada item berikutnya."),
    }

    println!("\n--- if let: Syntax Singkat untuk Match ---");
    // Terkadang, kita hanya peduli pada satu varian dan ingin mengabaikan sisanya.
    // `if let` adalah cara yang lebih singkat untuk melakukan ini.
    let nilai_favorit: Option<i32> = Some(7);

    // Kode ini hanya akan berjalan jika `nilai_favorit` adalah `Some`.
    if let Some(angka) = nilai_favorit {
        println!("Angka favorit saya adalah: {}", angka);
    }
    // Tidak perlu blok `else`, tetapi bisa ditambahkan jika perlu.
}
