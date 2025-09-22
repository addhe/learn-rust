use std::env; // Modul untuk berinteraksi dengan lingkungan, termasuk argumen CLI.

// Enum untuk merepresentasikan operasi yang valid.
// Menambahkan `Debug` memungkinkan kita untuk mencetak enum ini dengan mudah.
#[derive(Debug)]
enum Operasi {
    Tambah,
    Kurang,
    Kali,
    Bagi,
}

// Struct untuk menampung argumen yang sudah diparsing dan divalidasi.
#[derive(Debug)]
struct Argumen {
    angka1: f64,
    angka2: f64,
    operasi: Operasi,
}

// Implementasi method untuk struct Argumen.
impl Argumen {
    // Associated function (constructor) untuk mem-parsing argumen dari Vec<String>.
    // Fungsi ini bisa gagal, jadi ia mengembalikan Result.
    fn baru(args: &[String]) -> Result<Argumen, &'static str> {
        // Program CLI selalu memiliki setidaknya satu argumen (nama program itu sendiri).
        // Jadi kita butuh 4 total: `nama_program angka1 operasi angka2`.
        if args.len() < 4 {
            return Err("Error: Tidak cukup argumen. Contoh: cargo run -- 10 + 5");
        }

        // Parsing angka pertama.
        let angka1 = match args[1].parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err("Error: Argumen pertama bukan angka yang valid."),
        };

        // Parsing angka kedua.
        let angka2 = match args[3].parse::<f64>() {
            Ok(n) => n,
            Err(_) => return Err("Error: Argumen ketiga bukan angka yang valid."),
        };

        // Parsing operator.
        let operasi = match args[2].as_str() {
            "+" => Operasi::Tambah,
            "-" => Operasi::Kurang,
            "x" | "*" => Operasi::Kali, // Bisa 'x' atau '*'
            "/" => Operasi::Bagi,
            _ => return Err("Error: Operator tidak valid. Gunakan +, -, x, atau /."),
        };

        Ok(Argumen {
            angka1,
            angka2,
            operasi,
        })
    }
}

// Fungsi terpisah untuk melakukan kalkulasi berdasarkan argumen.
fn hitung(argumen: &Argumen) -> f64 {
    match argumen.operasi {
        Operasi::Tambah => argumen.angka1 + argumen.angka2,
        Operasi::Kurang => argumen.angka1 - argumen.angka2,
        Operasi::Kali => argumen.angka1 * argumen.angka2,
        Operasi::Bagi => argumen.angka1 / argumen.angka2,
    }
}

fn main() {
    // Mengumpulkan argumen dari command line menjadi sebuah Vector String.
    let args: Vec<String> = env::args().collect();

    // Mencoba membuat instance Argumen dari input.
    // `.unwrap_or_else()` adalah cara elegan untuk menangani Result.
    // Jika Ok, ia akan "membuka" nilainya.
    // Jika Err, ia akan menjalankan closure yang diberikan.
    let argumen = Argumen::baru(&args).unwrap_or_else(|err| {
        // Jika ada error saat parsing, cetak pesan error dan hentikan program.
        eprintln!("{}", err); // `eprintln!` mencetak ke standard error.
        std::process::exit(1);
    });

    // Jika parsing berhasil, lakukan perhitungan dan cetak hasilnya.
    let hasil = hitung(&argumen);
    println!(
        "Menghitung: {:?}... Hasilnya adalah: {}",
        argumen, hasil
    );
}
