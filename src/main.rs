mod gnome;

use gnome::{AtomicGnome, Gnome};
use std::{
    sync::{
        atomic::Ordering::{Relaxed, SeqCst},
        Arc, Mutex,
    },
    thread::{self},
    time::Duration,
};

fn main() {
    // Concurrency with data on atomic operations
    println!("ATOMIC\n");
    atomic();
    // Concurrency with achieved with
    println!("MUTEX GUARD\n");
    mutex();
}

fn atomic() {
    let skrzat = Arc::new(AtomicGnome::default());

    let _grow = thread::spawn({
        let skrzat = skrzat.clone();
        move || loop {
            //dont judge me here, I've just heard about being Relaxed or being SeqCst for the first time, also I've never been SeqCst
            let grow = skrzat.grow.load(Relaxed);

            if !grow {
                continue;
            }

            let age = skrzat.age.load(SeqCst);
            skrzat.age.store(age + 1, SeqCst);

            let age = skrzat.age.load(SeqCst);
            println!("{} of its atomic age", age);

            thread::sleep(Duration::from_millis(500));
        }
    });

    let _season = thread::spawn({
        let skrzat = skrzat.clone();
        move || loop {
            let age = skrzat.age.load(SeqCst);
            if age >= 11 {
                println!("Died of atomic age");
                break;
            }

            let grow = skrzat.grow.load(Relaxed);
            skrzat.grow.store(!grow, SeqCst);

            let grow = skrzat.grow.load(Relaxed);
            println!("Atomic growth {}", grow);

            thread::sleep(Duration::new(2, 0));
        }
    });

    loop {
        if _season.is_finished() {
            drop(skrzat);
            break;
        }
    }
}

fn mutex() {
    let skrzat = Arc::new(Mutex::new(Gnome::default()));

    // grow like every sleep
    let _grow = thread::spawn({
        let skrzat = skrzat.clone();
        move || loop {
            let mut guard = skrzat.lock().unwrap();

            if !guard.grow {
                continue;
            }

            guard.age += 1;
            println!("{} of age", guard.age);

            drop(guard);

            thread::sleep(Duration::from_millis(500));
        }
    });

    // grow on/off
    let _season = thread::spawn({
        let skrzat = skrzat.clone();
        move || loop {
            let mut guard = skrzat.lock().unwrap();

            if guard.age >= 11 {
                println!("Died of age");
                break;
            }

            guard.grow = !guard.grow;
            println!("Normie growth {}", guard.grow);

            drop(guard);
            thread::sleep(Duration::new(3, 0));
        }
    });

    loop {
        if _season.is_finished() {
            break;
        }
    }
}
