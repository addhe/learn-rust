**Silabus Belajar Bahasa Pemrograman Rust**

**Minggu 1: Pengenalan dan Instalasi**

1.  **Apa itu Rust?**
    *   Sejarah singkat Rust.
    *   Mengapa belajar Rust? (Keunggulan: performa, keamanan memori tanpa garbage collector, konkurensi).
    *   Fitur-fitur utama Rust.
2.  **Menyiapkan Lingkungan Pengembangan**
    *   **Instalasi Rust:**
        *   Menggunakan `rustup` (installer resmi Rust).
        *   Perintah instalasi untuk Windows, macOS, dan Linux.
    *   **Verifikasi Instalasi:**
        *   Mengecek versi `rustc --version`.
        *   Mengecek versi `cargo --version`.
    *   **Program "Hello, World!":**
        *   Menulis dan menjalankan program Rust pertama Anda.
    *   **Mengenal Cargo (Build System dan Package Manager Rust):**
        *   Membuat proyek baru dengan `cargo new nama_proyek`.
        *   Membangun proyek dengan `cargo build`.
        *   Menjalankan proyek dengan `cargo run`.
        *   Mengecek kode tanpa kompilasi dengan `cargo check`.

**Minggu 2: Dasar-Dasar Pemrograman Rust (Bagian 1)**

1.  **Variabel dan Mutabilitas**
    *   Deklarasi variabel dengan `let`.
    *   Konsep mutabilitas dengan kata kunci `mut`.
    *   Konstanta (`const`) dan perbedaannya dengan variabel immutable.
    *   Shadowing variabel.
2.  **Tipe Data Dasar (Scalar Types)**
    *   **Integer:** `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`. Literal angka.
    *   **Floating-Point:** `f32`, `f64`.
    *   **Boolean:** `bool` (true, false).
    *   **Character:** `char` (Unicode).
3.  **Tipe Data Gabungan (Compound Types)**
    *   **Tuple:** Kumpulan nilai dengan tipe yang berbeda, panjang tetap.
    *   **Array:** Kumpulan nilai dengan tipe yang sama, panjang tetap.
4.  **Fungsi (Functions)**
    *   Mendefinisikan dan memanggil fungsi.
    *   Parameter fungsi dan tipe data.
    *   Nilai kembali (return values) dari fungsi.
    *   Statements vs Expressions.
5.  **Komentar**
    *   Komentar satu baris (`//`).
    *   Komentar dokumentasi (`///` dan `//!`).

**Minggu 3: Dasar-Dasar Pemrograman Rust (Bagian 2)**

1.  **Kontrol Alur (Control Flow)**
    *   **Ekspresi `if/else`:** Kondisi dan percabangan.
    *   **`if let`:** Cara ringkas untuk menangani `Option` atau `Result`.
    *   **Perulangan (Loops):**
        *   `loop`: Perulangan tak terbatas (dengan `break`).
        *   `while`: Perulangan dengan kondisi.
        *   `for`: Perulangan untuk iterasi koleksi.

**Minggu 4: Konsep Kepemilikan (Ownership)**

1.  **Apa itu Ownership?**
    *   Masalah manajemen memori yang coba dipecahkan Rust.
    *   Tiga aturan dasar Ownership.
2.  **Stack dan Heap**
    *   Perbedaan cara data disimpan di memori.
3.  **Aturan Kepemilikan (Ownership Rules)**
    *   Setiap nilai di Rust memiliki variabel yang menjadi *pemiliknya*.
    *   Hanya ada satu pemilik pada satu waktu.
    *   Ketika pemilik keluar dari *scope*, nilai akan dihapus (`dropped`).
4.  **Referensi (References) dan Peminjaman (Borrowing)**
    *   Meminjam nilai tanpa mengambil alih kepemilikan.
    *   Referensi immutable (`&T`).
    *   Referensi mutable (`&mut T`) dan aturannya (hanya satu referensi mutable dalam satu scope tertentu).
    *   Dangling references dan bagaimana Rust mencegahnya.
5.  **Slice**
    *   Referensi ke sebagian dari koleksi (seperti String atau Array).
    *   String slices (`&str`).

**Minggu 5: Structs dan Enums**

1.  **Structs (Struktur Data)**
    *   Mendefinisikan dan membuat instance dari struct.
    *   Mengakses field struct.
    *   Tuple Structs (struct tanpa nama field).
    *   Unit-like Structs (struct tanpa field).
    *   Method pada Structs:
        *   Mendefinisikan method menggunakan blok `impl`.
        *   Method dengan parameter `&self`, `&mut self`, dan `self`.
        *   Associated functions (seperti konstruktor, contoh: `String::from`).
2.  **Enums (Enumerations)**
    *   Mendefinisikan enum untuk merepresentasikan pilihan.
    *   Varian enum dan data yang bisa disimpan.
    *   Enum `Option<T>`: Mengatasi nilai null atau ketiadaan nilai.
3.  **Pattern Matching dengan `match`**
    *   Operator kontrol alur yang kuat untuk mencocokkan pola.
    *   Menggunakan `match` dengan enum.
    *   Pola yang mengikat nilai.
    *   Placeholder `_`.
    *   `if let` sebagai sintaks ringkas untuk `match` pada satu pola.

**Minggu 6: Organisasi Kode dan Koleksi Umum**

1.  **Manajemen Proyek dengan Packages, Crates, dan Modules**
    *   **Packages dan Crates:** Struktur proyek Cargo.
    *   **Modules:** Mengelompokkan kode untuk organisasi dan privasi.
        *   Kata kunci `mod`.
        *   Visibilitas dengan `pub`.
    *   **Paths:** Cara merujuk item dalam hirarki modul.
    *   **Kata kunci `use`:** Membawa path ke dalam scope.
    *   Memisahkan modul ke dalam file yang berbeda.
2.  **Koleksi Umum (Common Collections)**
    *   **Vector (`Vec<T>`):** Daftar nilai yang bisa bertambah ukurannya, disimpan di heap.
        *   Membuat, membaca, mengubah, dan iterasi vector.
    *   **String (`String`):** Teks yang bisa dimodifikasi, disimpan di heap.
        *   Membuat, memperbarui, dan mengakses string.
        *   Perbedaan `String` dan `&str`.
    *   **Hash Map (`HashMap<K, V>`):** Menyimpan pasangan key-value.
        *   Membuat, mengakses, dan memperbarui hash map.

**Minggu 7: Penanganan Error (Error Handling)**

1.  **Error yang Tidak Dapat Dipulihkan dengan `panic!`**
    *   Kapan menggunakan `panic!`.
    *   Backtrace saat `panic!`.
2.  **Error yang Dapat Dipulihkan dengan `Result<T, E>`**
    *   Enum `Result` untuk operasi yang bisa gagal.
    *   Menangani `Result` dengan `match`.
    *   Shortcut untuk Propagating Errors: operator `?`.
3.  **Kapan Menggunakan `panic!` vs `Result`**.

**Langkah Selanjutnya Setelah Menguasai Dasar:**

*   **Membaca "The Rust Programming Language" (The Book):** Buku resmi dan sumber belajar utama Rust. (Tersedia online gratis).
*   **Mengerjakan Latihan Rustlings:** Latihan interaktif kecil untuk mempraktikkan konsep Rust.
*   **Membangun Proyek Kecil:** Terapkan pengetahuan Anda dengan membuat aplikasi sederhana.
*   **Mempelajari Konsep Lanjutan:**
    *   Generic Types, Traits, dan Lifetimes.
    *   Smart Pointers.
    *   Konkurensi (Fearless Concurrency).
    *   Testing.
    *   Dan banyak lagi!

**Tips Belajar:**

*   **Praktik Langsung:** Tulis kode sebanyak mungkin.
*   **Baca Kode Orang Lain:** Pelajari bagaimana programmer Rust lain menulis kode.
*   **Bergabung dengan Komunitas:** Forum Rust, Discord, atau grup lokal bisa sangat membantu.
*   **Jangan Takut Bertanya:** Komunitas Rust dikenal ramah dan suportif.

Selamat belajar Rust! Ini adalah bahasa yang kuat dan memuaskan untuk dipelajari.
