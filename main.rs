use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rdev::{listen, Event, EventType};
fn main() {
    env_logger::init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Spawn a thread to listen for keyboard events
    thread::spawn(move || {
        listen(move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                if key == rdev::Key::Escape {
                    r.store(false, Ordering::SeqCst);
                }
            }
        }).unwrap();
    });

    thread::sleep(Duration::from_secs(5));
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    while running.load(Ordering::SeqCst) {
        enigo.key(Key::Option, Press).unwrap();
        thread::sleep(Duration::from_millis(100)); // Add a small delay to avoid high CPU usage
    }

    enigo.key(Key::Option, Release).unwrap();
    println!("Exiting loop as Esc key was pressed");
}