// Fungsi bisa didefinisikan di mana saja, dan `main` adalah fungsi utama.
fn main() {
    println!("--- Memanggil Fungsi Lain ---");
    // Kita bisa memanggil fungsi lain yang sudah kita definisikan.
    sapa_pengguna("Alice", 25);
    sapa_pengguna("Bob", 30);

    println!("\n--- Fungsi dengan Nilai Balik (Return Value) ---");
    let hasil_penjumlahan = tambah_dua(10);
    println!("Hasil 10 + 2 adalah: {}", hasil_penjumlahan);

    // Rust adalah bahasa berbasis ekspresi. `if` bisa mengembalikan nilai.
    let angka = 7;
    let deskripsi_angka = if angka % 2 == 0 {
        "genap"
    } else {
        "ganjil"
    };
    println!("Angka {} adalah bilangan {}.", angka, deskripsi_angka);

    println!("\n--- Kontrol Alur: Perulangan (Loop) ---");
    demonstrasi_loop();
}

// === 1. Mendefinisikan Fungsi ===
// Ini adalah contoh fungsi yang menerima dua parameter.
// Kita harus mendeklarasikan tipe data untuk setiap parameter.
fn sapa_pengguna(nama: &str, umur: u32) {
    println!("Halo, {}! Anda berumur {} tahun.", nama, umur);
}

// === 2. Fungsi dengan Nilai Balik ===
// Kita bisa membuat fungsi yang mengembalikan nilai.
// Tipe nilai balik dideklarasikan setelah tanda panah `->`.
// Di Rust, baris terakhir dari sebuah fungsi, jika tidak diakhiri dengan titik koma (;),
// akan secara otomatis menjadi nilai balik (return value) dari fungsi tersebut.
// Ini disebut sebagai "ekspresi", bukan "statement".
fn tambah_dua(x: i32) -> i32 {
    x + 2 // Tidak ada titik koma, jadi ini adalah nilai yang dikembalikan.
}

// === 3. Berbagai Jenis Loop ===
fn demonstrasi_loop() {
    // a. `loop` - Perulangan tak terbatas
    // Akan terus berjalan sampai kita secara eksplisit menyuruhnya berhenti dengan `break`.
    let mut counter = 0;
    println!("a. Menjalankan 'loop'...");
    let hasil_loop = loop {
        counter += 1;
        print!("{} ", counter);
        if counter == 5 {
            break counter * 2; // `break` bisa mengembalikan nilai dari loop.
        }
    };
    println!("\n   Loop berhenti. Nilai yang dikembalikan dari loop: {}", hasil_loop);

    // b. `while` - Perulangan kondisional
    // Terus berjalan selama kondisi `true`.
    let mut angka = 3;
    println!("b. Menjalankan 'while'...");
    while angka != 0 {
        print!("{}! ", angka);
        angka -= 1;
    }
    println!("\n   Selesai!");

    // c. `for` - Perulangan untuk iterasi
    // Cara paling umum dan aman untuk melakukan iterasi atas sebuah koleksi,
    // seperti array atau range.
    let koleksi_angka = [10, 20, 30, 40, 50];
    println!("c. Menjalankan 'for' untuk iterasi array...");
    for elemen in koleksi_angka.iter() {
        print!("{} ", elemen);
    }
    println!();

    // `for` juga sangat berguna dengan `Range`.
    println!("   Menjalankan 'for' dengan range (1..4)...");
    for nomor in 1..4 { // Range dari 1 sampai 3 (4 tidak termasuk)
        print!("{} ", nomor);
    }
    println!();
}
