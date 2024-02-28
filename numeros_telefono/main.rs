use std::collections::BTreeMap;
use std::io;

fn main() {
    let numeros_tel = BTreeMap::from([
        ("2".to_string(), ["a", "b", "c", ""]),
        ("3".to_string(), ["d", "e", "f", ""]),
        ("4".to_string(), ["g", "h", "i", ""]),
        ("5".to_string(), ["j", "k", "l", ""]),
        ("6".to_string(), ["m", "n", "o", ""]),
        ("7".to_string(), ["p", "q", "r", "s"]),
        ("8".to_string(), ["t", "u", "v", ""]),
        ("9".to_string(), ["w", "x", "y", "z"]),
    ]);

    let mut resultado: Vec<String> = Vec::new();

    let mut ui = String::new();

    println!("Ingrese un String de números:");

    io::stdin().read_line(&mut ui).unwrap().to_string();

    // remove special characters
    ui = ui.replace(" ", "").replace("\n", "").replace("\r", "");
    println!("ui: {:?}", ui.to_string());

    let digits_array: Vec<char> = ui.chars().collect();
    println!("Digits array: {:?}", digits_array);

    // esto es para evitar index out of bounds

    if digits_array.len() == 0 {
    } else if digits_array.len() < 2 {
        for (clave, valor) in numeros_tel.iter() {
            if clave.to_string() == digits_array[0].to_string() {
                for i in 0..valor.len() {
                    if valor[i] != "" {
                        resultado.push(valor[i].to_string());
                    }
                }
            }
        }
    } else {
        for i in 0..digits_array.len() - 1 {
            let llave_actual = digits_array[i].to_string();
            println!("Llave actual: {:?}", llave_actual);

            for (clave, valor) in numeros_tel.iter() {
                if clave.to_string() == llave_actual {
                    for (clave_j, valor_j) in numeros_tel.iter() {
                        let siguiente_llave = digits_array[digits_array.len() - 1].to_string();

                        if clave_j.to_string() == siguiente_llave {
                            for i in 0..valor.len() {
                                for j in 0..valor_j.len() {
                                    if valor[i] != "" && valor_j[j] != "" && valor_j[j] != valor[i] {
                                        resultado.push(valor[i].to_string() + valor_j[j]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Resultado: {:?}", resultado);
}
