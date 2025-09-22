use std::num::ParseIntError;

// === 1. Fungsi yang Mengembalikan Result ===
// Fungsi ini mencoba menggandakan angka dari sebuah string.
// Operasi ini bisa gagal jika string tersebut bukan angka yang valid.
// Oleh karena itu, fungsi ini mengembalikan `Result<i32, ParseIntError>`.
// Jika sukses, ia mengembalikan `Ok(i32)`.
// Jika gagal, ia mengembalikan `Err(ParseIntError)`.
fn gandakan_angka_dari_string(teks: &str) -> Result<i32, ParseIntError> {
    // `teks.parse::<i32>()` sendiri sudah mengembalikan sebuah `Result`.
    match teks.parse::<i32>() {
        Ok(angka) => Ok(angka * 2), // Jika parsing sukses, gandakan angkanya.
        Err(e) => Err(e),           // Jika parsing gagal, teruskan error-nya.
    }
}

// === 2. Propagasi Error dengan Operator `?` ===
// Menulis `match` setiap saat bisa menjadi sangat bertele-tele.
// Rust menyediakan operator `?` sebagai jalan pintas untuk propagasi error.
// Jika `Result` adalah `Ok(nilai)`, `?` akan "membuka" `Result` dan mengembalikan `nilai`.
// Jika `Result` adalah `Err(e)`, `?` akan langsung menghentikan eksekusi fungsi saat ini
// dan mengembalikan `Err(e)` tersebut.
// PENTING: Operator `?` hanya bisa digunakan di dalam fungsi yang mengembalikan `Result` atau `Option`.
fn gandakan_angka_dengan_tanya(teks: &str) -> Result<i32, ParseIntError> {
    let angka = teks.parse::<i32>()?; // `?` menggantikan blok match di atas.
    Ok(angka * 2)
}

// Fungsi `main` juga bisa mengembalikan `Result`!
// Ini sangat berguna agar kita bisa menggunakan operator `?` di dalam `main`.
// `Box<dyn std::error::Error>` adalah "trait object" yang bisa menampung berbagai jenis error.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Menangani Result dengan `match` ---");
    let hasil1 = gandakan_angka_dari_string("10");
    match hasil1 {
        Ok(angka) => println!("Hasilnya adalah: {}", angka),
        Err(e) => println!("Terjadi error: {}", e),
    }

    let hasil2 = gandakan_angka_dari_string("halo");
    match hasil2 {
        Ok(angka) => println!("Hasilnya adalah: {}", angka),
        Err(e) => println!("Terjadi error: {}", e),
    }

    println!("\n--- Menangani Result dengan Operator `?` ---");
    // Karena `main` mengembalikan `Result`, kita bisa pakai `?` di sini.
    let hasil3 = gandakan_angka_dengan_tanya("25")?;
    println!("Hasil dari versi 'tanya' adalah: {}", hasil3);

    // Jika kita mencoba menggunakan `?` pada error, `main` akan berhenti dan mencetak error.
    // Uncomment baris di bawah untuk melihatnya.
    // let hasil4 = gandakan_angka_dengan_tanya("rust")?;
    // println!("Baris ini tidak akan pernah dijalankan: {}", hasil4);

    // Jika semuanya berjalan lancar, kita kembalikan Ok dari main.
    // `()` adalah "unit type", sebuah tuple kosong yang menandakan "tidak ada nilai".
    Ok(())
}
