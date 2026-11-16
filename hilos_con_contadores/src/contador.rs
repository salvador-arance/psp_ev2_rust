use std::sync::atomic::{AtomicI32, Ordering};

pub struct Contador {
    contador: AtomicI32,
}

impl Contador {
    pub fn new() -> Self {
        Self {
            contador: AtomicI32::new(0),
        }
    }

    pub fn incrementar_contador(&self) {
        self.contador.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_contador(&self) -> i32 {
        self.contador.load(Ordering::SeqCst)
    }
}