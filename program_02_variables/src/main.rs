fn main() {
    // === 1. VARIABEL IMMUTABLE (TIDAK BISA DIUBAH) ===
    // Secara default, variabel di Rust bersifat immutable. Artinya, setelah sebuah nilai
    // diberikan, nilai tersebut tidak bisa diubah lagi. Kita menggunakan keyword `let`.
    let x = 5;
    println!("1. Nilai dari x yang immutable adalah: {}", x);
    // Baris di bawah ini akan menyebabkan error saat kompilasi jika diaktifkan:
    // x = 6; // error: cannot assign twice to immutable variable `x`
    // Ini adalah salah satu fitur keamanan utama Rust untuk mencegah bug.

    // === 2. VARIABEL MUTABLE (BISA DIUBAH) ===
    // Untuk membuat variabel yang nilainya bisa diubah, kita harus secara eksplisit
    // menambahkan keyword `mut` (kependekan dari mutable).
    let mut y = 10;
    println!("2. Nilai awal dari y yang mutable adalah: {}", y);
    y = 15; // Ini valid karena y adalah mutable.
    println!("   Nilai baru dari y adalah: {}", y);

    // === 3. SHADOWING ===
    // Rust memperbolehkan kita mendeklarasikan variabel baru dengan nama yang sama
    // dengan variabel sebelumnya. Ini disebut "shadowing". Ini berbeda dengan mutabilitas
    // karena kita sebenarnya membuat variabel *baru*.
    // Shadowing sangat berguna karena kita bisa mengubah tipe data dari sebuah variabel.
    let z = 20; // z adalah integer (i32)
    println!("3. Nilai z adalah: {}", z);
    let z = "dua puluh"; // z sekarang di-"shadow" oleh variabel baru yang tipenya string.
    println!("   Nilai z yang sudah di-shadow adalah: {}", z);

    // === 4. TIPE DATA PRIMITIF ===
    // Rust adalah bahasa yang "statically typed", artinya tipe setiap variabel harus
    // diketahui saat kompilasi. Namun, seringkali kita tidak perlu menuliskannya
    // secara eksplisit karena compiler cukup pintar untuk menentukannya (type inference).

    // Tipe Integer (Bilangan Bulat)
    // i32 adalah default. 'i' untuk signed (bisa negatif), 'u' untuk unsigned (hanya positif).
    let angka_bulat: i32 = -500;
    println!("4. Sebuah integer: {}", angka_bulat);

    // Tipe Floating-Point (Bilangan Desimal)
    // f64 adalah default dan memiliki presisi ganda.
    let angka_desimal: f64 = 3.14;
    println!("   Sebuah float: {}", angka_desimal);

    // Tipe Boolean (bool)
    // Hanya bisa bernilai `true` atau `false`.
    let apakah_rust_keren: bool = true;
    println!("   Apakah Rust keren? {}", apakah_rust_keren);

    // Tipe Karakter (char)
    // Mewakili satu karakter Unicode. Ditulis dengan kutip tunggal ('').
    let sebuah_karakter: char = '🦀'; // Kepiting adalah maskot Rust!
    println!("   Sebuah karakter: {}", sebuah_karakter);
}
