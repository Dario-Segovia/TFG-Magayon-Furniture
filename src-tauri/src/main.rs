// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bcrypt::verify;


use sqlx::postgres::PgPoolOptions;

use std::env;

mod empleados;


#[tauri::command]
async fn login(usuario: String, password: String) -> Result<String, String> {
    // Carga las variables de entorno desde el archivo .env
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    // Establece la conexión con la base de datos PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| format!("Error al conectar a la base de datos: {}", e))?;

    // Consulta el usuario y la contraseña cifrada en la base de datos
    let row: (String, String) = sqlx::query_as("SELECT password, rol FROM public.usuarios WHERE usuario = $1")
        .bind(&usuario)
        .fetch_one(&pool)
        .await
        .map_err(|e| format!("Usuario no encontrado o error en la consulta: {}", e))?;

    // Desencripta la contraseña y compara
    let (hashed_password, rol) = row;
    if verify(&password, &hashed_password).map_err(|e| format!("Error al verificar la contraseña: {}", e))? {
        Ok(rol) // Retorna el rol si la contraseña es correcta
    } else {
        Err("Credenciales incorrectas".to_string()) // Si la contraseña es incorrecta
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        login,
        empleados::create_employee,
        empleados::get_employees,
        empleados::update_employee,
        empleados::delete_employee
    ])
         // Esto conecta el comando de Rust con el frontend
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
