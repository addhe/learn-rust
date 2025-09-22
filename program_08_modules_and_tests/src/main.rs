// Kita menggunakan `use` untuk membawa modul `kalkulator` dari pustaka kita ke dalam scope.
// Nama pustaka sama dengan nama paket/proyek.
use program_08_modules_and_tests::kalkulator;

fn main() {
    let hasil_tambah = kalkulator::tambah(10, 20);
    let hasil_kurang = kalkulator::kurang(20, 5);

    println!("Selamat datang di program utama!");
    println!("Menggunakan pustaka `kalkulator`:");
    println!("10 + 20 = {}", hasil_tambah);
    println!("20 - 5 = {}", hasil_kurang);
    println!("\nUntuk menjalankan unit test, gunakan perintah `cargo test`");
}
