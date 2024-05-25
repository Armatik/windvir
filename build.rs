use std::path::Path;
use std::fs;


fn run_above_dir(path: &str, c_files: &mut Vec::<String>) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            let format = path.split(".").collect::<Vec<&str>>();
            let format = match format.get(format.len() - 1) {
                Some(format) => *format,
                None => return,
            };

            if format == "h" || format == "c" {
                let canon = fs::canonicalize(path).expect("Ошибка! Не удалось канонизировать путь");
                let canon = canon.to_str().expect("Ошибка! Не удалось перевести полный путь в строку");

                c_files.push(canon.to_string());
            }

            return;
        },
    };

    for doc in dir {
        if let Ok(doc) = doc {
            run_above_dir(doc.path().to_str().expect("Ошибка! Не удалось перевести путь в строку"), c_files);
        }
    }
}


fn main() {
    let dst = Path::new(".");

    let mut cfg = cc::Build::new();
    cfg.cuda(false);
    #[cfg(unix)]
    {
        cfg.include("./include")
            .file("./src/ffi/trans.c")
            .out_dir(dst.join("lib"))
            .flag("-O3")
            .compile("libwindvir.a");
    }

    #[cfg(windows)]
    {
        cfg.include("./include")
            .file("./src/ffi/trans.c")
            .out_dir(dst.join("lib"))
            .compile("libwindvir.a");
    }

    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", dst.join("include").display());
    
    let mut c_files = Vec::<String>::new();

    run_above_dir("./", &mut c_files);

    for file in c_files {
        println!("cargo:rerun-if-changed={file}");
    }
}
