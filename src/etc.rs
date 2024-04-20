pub fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Ожидался вектор с длинной {}, но он был динной {}", N, v.len()))
}


pub fn print_help() {
    println!(
        "WindVir - программа для простейшей симуляции ветра среди 2D объектов.
\t-c - Запустить программу c FFI режимом.
\t-r - Запустить программу с разноцветным фоном."
    );
}
