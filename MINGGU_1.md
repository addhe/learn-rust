\
# Minggu 1: Petualangan Dimulai - Kenalan dengan Rust!

Selamat datang, Ksatria Kode Pemberani! Kamu baru saja menginjakkan kaki di gerbang sebuah dunia baru yang menakjubkan: dunia Pemrograman Rust. Anggap saja ini adalah minggu pertamamu di akademi sihir kode, dan pelajaran pertama kita adalah mengenal "mantra" dasar dan menyiapkan "tongkat sihir" kita.

## Bab 1: Apa Sih Rust Itu? Kenapa Harus Peduli?

Bayangkan sebuah kerajaan digital yang sering diserang oleh monster-monster jahat bernama "Bug Memori" dan "Lambatnya Performa". Nah, Rust adalah ksatria pelindung super canggih yang diciptakan oleh para penyihir di Mozilla Research (mereka juga yang membuat browser Firefox, lho!). Ksatria ini lahir sekitar tahun 2010 dan resmi diperkenalkan ke publik pada tahun 2015.

**Kenapa kita harus berteman dengan Ksatria Rust ini?**

*   **Super Cepat (Performa):** Rust bisa membuat programmu berlari secepat kilat, setara dengan bahasa legendaris seperti C++. Cocok untuk membangun game, sistem operasi, atau aplikasi yang butuh kecepatan tinggi.
*   **Super Aman (Keamanan Memori):** Ini dia jurus andalan Rust! Dia punya sistem "kepemilikan" yang unik. Bayangkan setiap "harta karun" (data) di programmu dijaga ketat. Rust memastikan tidak ada yang bisa mencuri atau merusaknya secara tidak sengaja, tanpa perlu "tukang sampah" (garbage collector) yang kadang bikin program jadi lambat. Ini berarti lebih sedikit begadang mencari bug aneh!
*   **Jago Kerja Bareng (Konkurensi):** Rust memudahkanmu membuat program yang bisa mengerjakan banyak hal sekaligus dengan aman, tanpa takut tabrakan data.

Singkatnya, Rust itu seperti membangun kastil yang kokoh, cepat, dan punya banyak penjaga pintar sekaligus!

## Bab 2: Menyiapkan Peralatan Perang (Instalasi Rust)

Setiap ksatria butuh pedang dan perisai. Di dunia Rust, "pedang" kita adalah `rustc` (Rust Compiler, si penerjemah mantra) dan "perisai" sekaligus "pembantu setia" kita adalah `cargo` (Manajer Proyek dan Paket).

**Langkah 1: Memanggil `rustup` sang Penempa Senjata**

Cara terbaik untuk mendapatkan peralatan Rust adalah melalui `rustup`. Ini adalah installer resmi yang akan mengurus semuanya.

*   Buka "Terminal" atau "Command Prompt" di komputermu (ini seperti jendela ajaib untuk memberi perintah).
*   Ketik mantra berikut dan tekan Enter (mantra spesifiknya bisa kamu lihat di website resmi Rust, [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install), karena bisa sedikit berbeda untuk Windows, macOS, atau Linux). Biasanya sih cuma satu baris perintah!
*   Ikuti petunjuk yang muncul di layar. `rustup` akan mengunduh dan memasang semua yang dibutuhkan.

**Langkah 2: Memastikan Senjata Sudah Tajam (Verifikasi Instalasi)**

Setelah `rustup` selesai, kita perlu cek apakah "pedang" dan "perisai" kita sudah siap.

*   Di Terminal, ketik:
    ```bash
    rustc --version
    ```
    Kamu akan melihat versi `rustc` terpasang.
*   Lalu ketik:
    ```bash
    cargo --version
    ```
    Kamu akan melihat versi `cargo` terpasang.

Jika kedua perintah ini menunjukkan versi, selamat! Peralatanmu sudah siap tempur!

## Bab 3: Mantra Pertama - "Halo, Dunia!"

Saatnya mencoba sihir pertamamu! Kita akan membuat program sederhana yang menyapa dunia.

1.  **Buat Folder Proyek:** Buat folder baru di komputermu, misalnya `proyek_rust_pertamaku`.
2.  **Buat File Mantra:** Di dalam folder itu, buat file baru bernama `main.rs` (ekstensi `.rs` adalah ciri khas file kode Rust).
3.  **Tulis Mantra:** Buka `main.rs` dengan editor teks (seperti VS Code, Notepad++, Sublime Text, dll.) dan tulis mantra ini:

    ```rust
    fn main() {
        println!("Halo, Dunia Rust!");
    }
    ```
    *   `fn main()`: Ini adalah mantra utama yang akan dijalankan pertama kali. Semua program Rust harus punya ini.
    *   `println!("Halo, Dunia Rust!");`: Ini adalah mantra untuk mencetak teks ke layar. Tanda `!` menunjukkan ini adalah "macro", semacam mantra spesial di Rust.

4.  **Kompilasi Mantra (Mengubah Mantra Jadi Kekuatan Nyata):**
    *   Kembali ke Terminal, masuk ke direktori `proyek_rust_pertamaku` (gunakan perintah `cd nama_folder`).
    *   Ketik:
        ```bash
        rustc main.rs
        ```
        Ini akan memanggil `rustc` untuk menerjemahkan kodemu menjadi program yang bisa dijalankan komputer. Jika berhasil, akan muncul file baru (misalnya `main` atau `main.exe`).

5.  **Jalankan Program:**
    *   Di Terminal, ketik:
        ```bash
        ./main  # Untuk macOS/Linux
        # atau
        main.exe # Untuk Windows
        ```
    *   Voila! Kamu akan melihat tulisan "Halo, Dunia Rust!" muncul. Sihir pertamamu berhasil!

## Bab 4: Berteman dengan Cargo, Si Asisten Serbaguna

Meskipun kita bisa kompilasi manual seperti di atas, untuk proyek yang lebih besar, kita butuh bantuan `cargo`. Cargo ini super pintar!

1.  **Membuat Proyek dengan Cargo:**
    *   Buka Terminal di tempat kamu ingin membuat proyek baru (jangan di dalam `proyek_rust_pertamaku` lagi ya).
    *   Ketik:
        ```bash
        cargo new halo_cargo
        ```
        Ganti `halo_cargo` dengan nama proyekmu.
    *   Cargo akan membuatkan struktur folder yang rapi untukmu, termasuk file `src/main.rs` yang sudah berisi kode "Hello, world!" versi Cargo, dan file `Cargo.toml` (file konfigurasi proyek).

2.  **Melihat Isi `src/main.rs`:**
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```
    Sama kan? Tapi sekarang diatur oleh Cargo.

3.  **Membangun Proyek dengan Cargo (`cargo build`):**
    *   Masuk ke direktori `halo_cargo` di Terminal (`cd halo_cargo`).
    *   Ketik:
        ```bash
        cargo build
        ```
        Cargo akan mengompilasi proyekmu dan menyimpan hasilnya di folder `target/debug/`.

4.  **Menjalankan Proyek dengan Cargo (`cargo run`):**
    *   Masih di direktori `halo_cargo`, ketik:
        ```bash
        cargo run
        ```
        Cargo akan membangun (jika belum) DAN langsung menjalankan programmu. Praktis!

5.  **Mengecek Kode Tanpa Kompilasi Penuh (`cargo check`):**
    *   Kadang kita cuma mau cek apakah ada kesalahan di kode tanpa perlu menunggu proses kompilasi yang lama. Ketik:
        ```bash
        cargo check
        ```
        Ini jauh lebih cepat dan bagus untuk memeriksa kesalahan saat kamu sedang menulis kode.

Wow, minggu pertama yang luar biasa! Kamu sudah mengenal Rust, menyiapkan peralatanmu, mengucapkan mantra pertama, dan bahkan berteman dengan Cargo si asisten handal.

Di minggu berikutnya, kita akan mulai mempelajari "bahan-bahan" dasar sihir Rust seperti variabel, tipe data, dan fungsi. Sampai jumpa di petualangan selanjutnya, Ksatria Kode!
