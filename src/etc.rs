pub fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Ожидался вектор с длинной {}, но он был динной {}", N, v.len()))
}
