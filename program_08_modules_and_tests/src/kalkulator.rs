// Kita membuat semua fungsi di sini publik (`pub`) agar bisa diakses dari luar modul.
pub fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

pub fn kurang(a: i32, b: i32) -> i32 {
    a - b
}

// `#[cfg(test)]` adalah atribut yang memberitahu compiler Rust
// untuk hanya meng-compile dan menjalankan kode berikut ketika kita
// menjalankan perintah `cargo test`, bukan `cargo build` atau `cargo run`.
#[cfg(test)]
mod tests {
    // `use super::*;` membawa semua item dari modul induk (yaitu `kalkulator`)
    // ke dalam scope modul `tests`.
    use super::*;

    // `#[test]` adalah atribut yang menandakan bahwa ini adalah sebuah fungsi test.
    #[test]
    fn test_tambah() {
        // `assert_eq!` adalah makro yang memeriksa apakah dua nilai sama.
        // Jika tidak, test akan gagal (panic).
        assert_eq!(tambah(2, 2), 4);
        assert_eq!(tambah(-1, 5), 4);
    }

    #[test]
    fn test_kurang() {
        assert_eq!(kurang(10, 5), 5);
        assert_ne!(kurang(10, 5), 6); // `assert_ne!` memeriksa ketidaksamaan.
    }

    #[test]
    #[should_panic] // Atribut ini menandakan bahwa kita mengharapkan test ini untuk panic.
    fn test_penjumlahan_overflow() {
        // Penjumlahan integer di Rust akan panic dalam mode debug jika terjadi overflow.
        // Test ini sengaja dibuat untuk menunjukkan hal tersebut.
        tambah(i32::MAX, 1);
    }
}
