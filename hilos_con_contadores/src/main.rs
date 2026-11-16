mod contador;
mod incrementa_contadores;

use contador::Contador;
use incrementa_contadores::IncrementaContadores;
use std::sync::Arc;
use std::thread;

/// Hello world!
fn main() {
    let contador = Arc::new(Contador::new());

    let tarea1 = IncrementaContadores::new(Arc::clone(&contador));
    let tarea2 = IncrementaContadores::new(Arc::clone(&contador));

    let t1 = thread::spawn(move || tarea1.run());
    let t2 = thread::spawn(move || tarea2.run());

    if let Err(e) = t1.join() {
        eprintln!("{:?}", e);
    }
    if let Err(e) = t2.join() {
        eprintln!("{:?}", e);
    }

    println!("{}", contador.get_contador()); // 300000
}