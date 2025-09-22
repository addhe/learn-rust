fn main() {
    // === 1. OWNERSHIP & MOVE ===
    // String adalah tipe data yang dialokasikan di 'heap', jadi ini contoh yang bagus.
    let s1 = String::from("hello");
    println!("1. s1 dibuat, nilainya: '{}'", s1);

    // Ketika kita "menugaskan" s1 ke s2, kepemilikan s1 dipindahkan (moved) ke s2.
    // Ini bukan "shallow copy". Setelah ini, s1 tidak lagi valid.
    let s2 = s1;
    println!("   s1 dipindahkan ke s2, nilai s2: '{}'", s2);

    // Baris di bawah ini akan menyebabkan COMPILE ERROR jika diaktifkan,
    // karena kepemilikan s1 sudah pindah ke s2. s1 sudah tidak bisa diakses.
    // println!("Nilai s1 setelah move: {}", s1); // error[E0382]: borrow of moved value: `s1`

    // Hal yang sama terjadi saat memanggil fungsi.
    let s3 = String::from("dunia");
    println!("   s3 dibuat, nilainya: '{}'", s3);
    ambil_kepemilikan(s3); // Kepemilikan s3 pindah ke dalam fungsi.
    // s3 tidak lagi valid di sini.

    // === 2. CLONE ===
    // Jika kita benar-benar ingin membuat duplikat (deep copy) dari data di heap,
    // kita bisa menggunakan method `.clone()`.
    let s4 = String::from("clone saya");
    let s5 = s4.clone();
    println!("\n2. s4 di-clone menjadi s5.");
    println!("   s4 = {}, s5 = {}", s4, s5); // Keduanya valid karena s5 adalah salinan.

    // === 3. BORROWING & REFERENCES ===
    // Daripada memindahkan kepemilikan, kita bisa "meminjamkannya" dengan referensi.
    // Referensi ditandai dengan ampersand (&).
    let s6 = String::from("pinjam saya");
    println!("\n3. s6 dibuat, nilainya: '{}'", s6);

    // Kita meminjamkan s6 ke fungsi, bukan memindahkan kepemilikan.
    let panjang = hitung_panjang(&s6);
    println!("   Panjang dari s6 adalah {}. s6 masih valid: '{}'", panjang, s6); // s6 masih bisa digunakan!

    // === 4. MUTABLE REFERENCES ===
    // Kita juga bisa meminjam referensi yang mutable (`&mut`) untuk mengubah data.
    let mut s7 = String::from("ubah");
    println!("\n4. s7 dibuat, nilainya: '{}'", s7);
    ubah_string(&mut s7);
    println!("   Setelah diubah, nilai s7: '{}'", s7);

    // ATURAN PENTING REFERENSI:
    // Dalam satu scope, Anda hanya boleh memiliki:
    // - SATU referensi mutable (&mut T), ATAU
    // - BEBERAPA referensi immutable (&T).
    // Anda tidak bisa memiliki referensi mutable jika ada referensi immutable.
    // Ini adalah cara Rust mencegah "data races" saat kompilasi.
}

// Fungsi ini "mengambil alih kepemilikan" dari string yang diberikan.
fn ambil_kepemilikan(some_string: String) {
    println!("   Di dalam fungsi `ambil_kepemilikan`, nilainya: '{}'", some_string);
} // `some_string` keluar dari scope di sini, dan memorinya dibebaskan.

// Fungsi ini "meminjam" sebuah string dan tidak mengambil kepemilikan.
fn hitung_panjang(s: &String) -> usize {
    s.len()
} // `s` keluar dari scope, tetapi karena tidak memiliki kepemilikan, tidak ada yang terjadi.

// Fungsi ini meminjam referensi mutable untuk mengubah string.
fn ubah_string(s: &mut String) {
    s.push_str(" saya");
}
