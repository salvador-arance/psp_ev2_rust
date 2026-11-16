use crate::contador::Contador;
use std::sync::Arc;

pub struct IncrementaContadores {
    contador: Arc<Contador>,
}

impl IncrementaContadores {
    pub fn new(contador: Arc<Contador>) -> Self {
        Self { contador }
    }

    pub fn run(&self) {
        for _ in 0..150_000 {
            self.contador.incrementar_contador();
        }
    }
}