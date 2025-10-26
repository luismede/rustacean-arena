fn pesquisa_binaria(vec: &Vec<i32>, item: i32) -> Option<usize> {
    let mut baixo = 0;
    let mut alto = vec.len();

    while baixo < alto {
        let meio = baixo + (baixo + alto) / 2;
        let chute = vec[meio];
        if chute == item {
            return Some(meio);
        }
        if chute > item {
            alto = meio;
        } else {
            baixo = meio + 1;
        }
    }
    return None;
}

fn main() {
    let minha_lista = vec![1, 3, 5, 7, 9];
    println!("{:?}", pesquisa_binaria(&minha_lista, 3));
    println!("{:?}", pesquisa_binaria(&minha_lista, -1));
}
