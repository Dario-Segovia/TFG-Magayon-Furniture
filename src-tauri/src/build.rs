#[allow(dead_code)]

fn main() {
    // Asegura que el build se vuelva a ejecutar si cambia el .env
    println!("cargo:rerun-if-changed=.env");

    // Carga el .env en tiempo de compilación
    if let Ok(dotenv_path) = dotenvy::dotenv() {
        println!("cargo:rerun-if-changed={}", dotenv_path.display());
    }

    // Inyecta DATABASE_URL en el binario
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        println!("cargo:rustc-env=DATABASE_URL={}", db_url);
    } else {
        panic!("DATABASE_URL no está definido en .env");
    }
}
