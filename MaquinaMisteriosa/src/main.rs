use std::io;
use std::cmp::min;

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Error al leer la línea");
    let mut numbers = input_line.split_whitespace();
    let n: u32 = numbers
        .next()
        .expect("No se encontró el primer número (n)")
        .parse()
        .expect("No se pudo convertir 'n' a un número");
    let m: u32 = numbers
        .next()
        .expect("No se encontró el segundo número (m)")
        .parse()
        .expect("No se pudo convertir 'm' a un número");
    println!("{}", maquinaMisteriosa(n, m));
    
}

fn maquinaMisteriosa(n: u32, mut m: u32) -> u32{

    if n >= m {
        return n - m;
    }

    let mut pasos = 0;
    while m > n {
        if m%2 == 0 {
            m = m/2;
        }
        else {
            m = m+1;
        }
        pasos += 1;

    }
    return pasos + n - m;
}