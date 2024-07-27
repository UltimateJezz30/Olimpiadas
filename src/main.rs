use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct Patinador {
    nombre: String,
    calificaciones: Vec<f64>,
}

impl Patinador {
    fn new(nombre: String, calificaciones: Vec<f64>) -> Self {
        Patinador { nombre, calificaciones }
    }
}

fn leer_archivo(filename: &str) -> io::Result<Vec<Patinador>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut patinadores = Vec::new();
    let mut lines = reader.lines();

    if let Some(Ok(_)) = lines.next() {
        for line in lines {
            if let Ok(line) = line {
                let partes: Vec<&str> = line.split(',').collect();
                let nombre = partes[0].trim().to_string();
                let calificaciones = partes[1..]
                    .iter()
                    .map(|x| x.trim().parse::<f64>().unwrap())
                    .collect();
                patinadores.push(Patinador::new(nombre, calificaciones));
            }
        }
    }

    Ok(patinadores)
}

fn calcular_puntaje_final(puntajes: &[f64]) -> f64 {
    let (max_puntaje, min_puntaje) = puntajes.iter().fold((f64::NEG_INFINITY, f64::INFINITY), |(max, min), &x| {
        (max.max(x), min.min(x))
    });

    let suma_total: f64 = puntajes.iter().sum();
    (suma_total - max_puntaje - min_puntaje) / (puntajes.len() as f64 - 2.0)
}

fn main() -> io::Result<()> {
    let competidores = leer_archivo("Calificación.txt")?;

    let mut output_file = File::create("Puntaje Final.txt")?;
    for patinador in &competidores {
        let puntaje_artistico: Vec<f64> = patinador.calificaciones.iter().take(3).cloned().collect();
        let puntaje_tecnico: Vec<f64> = patinador.calificaciones.iter().skip(3).cloned().collect();

        let puntaje_artistico_final = calcular_puntaje_final(&puntaje_artistico);
        let puntaje_tecnico_final = calcular_puntaje_final(&puntaje_tecnico);

        println!("Patinador: {}, Puntaje Artístico: {:.2}, Puntaje Técnico: {:.2}", patinador.nombre, puntaje_artistico_final, puntaje_tecnico_final);

        let puntaje_final = calcular_puntaje_final(&patinador.calificaciones);
        writeln!(output_file, "{}: {:.2}", patinador.nombre, puntaje_final)?;
    }

    Ok(())
}
