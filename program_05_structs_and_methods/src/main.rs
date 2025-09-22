// Pertama, kita definisikan sebuah struct.
// Ini seperti cetak biru untuk data kita.
// Struct ini akan merepresentasikan sebuah Persegi Panjang.
struct PersegiPanjang {
    panjang: u32,
    lebar: u32,
}

// Selanjutnya, kita membuat blok implementasi (`impl`) untuk struct kita.
// Di sinilah kita mendefinisikan method-method yang berhubungan dengan PersegiPanjang.
impl PersegiPanjang {
    // Method pertama kita adalah `luas`.
    // Method ini "meminjam" struct secara immutable (&self).
    // `&self` adalah kependekan dari `self: &Self`, di mana `Self` adalah alias untuk `PersegiPanjang`.
    // Kita hanya perlu membaca data, jadi pinjaman immutable sudah cukup.
    fn luas(&self) -> u32 {
        self.panjang * self.lebar
    }

    // Method ini juga meminjam secara immutable.
    fn bisa_menampung(&self, lain: &PersegiPanjang) -> bool {
        self.panjang > lain.panjang && self.lebar > lain.lebar
    }

    // Method ini meminjam secara mutable (&mut self).
    // Kita perlu meminjam secara mutable karena kita akan mengubah nilai dari struct.
    fn gandakan_lebar(&mut self) {
        self.lebar *= 2;
    }

    // Ini bukan method, tetapi sebuah "associated function" (fungsi terasosiasi).
    // Disebut begitu karena tidak menerima `self` sebagai parameter.
    // Fungsi seperti ini sering digunakan sebagai "constructor" untuk membuat instance baru.
    // Kita memanggilnya dengan `PersegiPanjang::persegi(ukuran)`.
    fn persegi(ukuran: u32) -> PersegiPanjang {
        PersegiPanjang {
            panjang: ukuran,
            lebar: ukuran,
        }
    }
}

fn main() {
    // Membuat instance dari struct PersegiPanjang.
    let mut pp1 = PersegiPanjang {
        panjang: 50,
        lebar: 30,
    };

    // Menggunakan method `luas` yang kita definisikan di blok `impl`.
    println!(
        "1. Luas dari persegi panjang dengan panjang {} dan lebar {} adalah {}.",
        pp1.panjang,
        pp1.lebar,
        pp1.luas() // Memanggil method dengan syntax titik.
    );

    // Menggunakan method `gandakan_lebar`.
    println!("\n2. Menggandakan lebar...");
    pp1.gandakan_lebar(); // Meminjam pp1 secara mutable.
    println!(
        "   Sekarang, luasnya menjadi: {}.",
        pp1.luas()
    );


    let pp2 = PersegiPanjang {
        panjang: 40,
        lebar: 20,
    };
    let pp3 = PersegiPanjang {
        panjang: 60,
        lebar: 40,
    };

    println!("\n3. Membandingkan persegi panjang...");
    println!("   Apakah pp1 bisa menampung pp2? {}", pp1.bisa_menampung(&pp2));
    println!("   Apakah pp1 bisa menampung pp3? {}", pp1.bisa_menampung(&pp3));

    // Menggunakan associated function `persegi` untuk membuat instance baru.
    let persegi_keren = PersegiPanjang::persegi(25);
    println!("\n4. Membuat persegi menggunakan 'associated function'...");
    println!(
        "   Luas dari persegi dengan sisi {} adalah {}.",
        persegi_keren.panjang,
        persegi_keren.luas()
    );
}
