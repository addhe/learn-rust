use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // === 1. MEMBUAT THREAD SEDERHANA ===
    println!("--- 1. Thread Sederhana ---");

    // `thread::spawn` membuat thread baru.
    // "Closure" (fungsi anonim `|| { ... }`) yang diberikan akan dijalankan di thread tersebut.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread yang di-spawn: hitungan ke-{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Sementara itu, thread utama (main) juga terus berjalan.
    for i in 1..=3 {
        println!("Thread utama: hitungan ke-{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // `handle.join()` akan menunggu sampai thread yang di-spawn selesai.
    // Jika tidak ada `.join()`, thread utama bisa saja selesai lebih dulu
    // dan mematikan program sebelum thread baru sempat menyelesaikan tugasnya.
    handle.join().unwrap();

    // === 2. BERBAGI DATA ANTAR THREAD DENGAN AMAN ===
    println!("\n--- 2. Berbagi Data dengan Arc<Mutex<T>> ---");

    // Kita ingin beberapa thread sama-sama menaikkan nilai sebuah counter.
    // Untuk melakukan ini dengan aman, kita butuh:
    // - `Mutex<T>`: (Mutual Exclusion) Memastikan hanya satu thread yang bisa
    //   mengakses data pada satu waktu.
    // - `Arc<T>`: (Atomically Reference Counted) Cara untuk memiliki beberapa "pemilik"
    //   dari data yang sama di berbagai thread. `Arc` adalah versi thread-safe dari `Rc`.

    // Kita membungkus data kita (sebuah angka `i32`) dalam Mutex, lalu dalam Arc.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Membuat 10 thread.
    for i in 0..10 {
        // `Arc::clone()` membuat referensi baru ke data yang sama. Ini sangat cepat.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // `lock()` akan "mengunci" mutex, memblokir thread lain sampai kita selesai.
            // Ia mengembalikan sebuah "lock guard" (smart pointer) yang memberi kita akses mutable.
            let mut num = counter_clone.lock().unwrap();

            // Sekarang kita bisa mengubah data dengan aman.
            *num += 1;
            println!("Thread {} menaikkan counter menjadi {}", i, *num);

            // Ketika `num` (lock guard) keluar dari scope di akhir closure ini,
            // kunci akan secara otomatis dilepaskan. Ini sangat aman dan praktis (prinsip RAII).
        });
        handles.push(handle);
    }

    // Menunggu semua thread selesai.
    for handle in handles {
        handle.join().unwrap();
    }

    // Mencetak hasil akhir. Kita perlu mengunci mutex lagi untuk membaca nilainya.
    println!("\nSetelah semua thread selesai, hasil akhir counter adalah: {}", *counter.lock().unwrap());
}
