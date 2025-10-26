use std::vec;

fn buscar_menor(vec: &Vec<i32>) -> Option<usize> {
    if vec.is_empty() {
        return None;
    }

    let mut menor = vec[0];
    let mut menor_indice = 0;

    for i in 1..vec.len() {
        if vec[i] < menor {
            menor = vec[i];
            menor_indice = i;
        }
    }
    Some(menor_indice)
}

fn ordenacao_por_selecao(arr: &Vec<i32>) -> Vec<i32> {
    let mut arr_clonado = arr.clone();
    let mut novo_vec = Vec::with_capacity(arr.len());

    while !arr_clonado.is_empty() {
        let menor_indice = buscar_menor(&arr_clonado).unwrap();
        let menor_valor = arr_clonado.remove(menor_indice);

        novo_vec.push(menor_valor);
    }
    novo_vec
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let numeros_ordenados = ordenacao_por_selecao(&numeros);

    println!("Vetor original: {:?}", numeros);
    println!("Vetor ordenado: {:?}", numeros_ordenados);
}
