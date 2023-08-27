use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Stop {
    stop: Arc<AtomicBool>,
}

impl Stop {
    pub fn new() -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let s = stop.clone();
        ctrlc::set_handler(move || {
            s.store(true, Ordering::Relaxed);
            println!("\nReceived Ctrl-C! Stop and Exit...");
        })
        .expect("Error setting Ctrl-C handler");
        Self { stop }
    }

    pub fn stopped(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }
}
