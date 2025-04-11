#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bcrypt::verify;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tauri::State;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

mod empleados;
mod horarios;

// -----------------------------
// FUNCIONES DE LOGIN
// -----------------------------

#[tauri::command]
async fn login(usuario: String, password: String, pool: State<'_, PgPool>) -> Result<String, String> {
    // Consulta el usuario y la contraseña cifrada en la base de datos
    let row: (String, String) = sqlx::query_as("SELECT password, rol FROM public.usuarios WHERE usuario = $1")
        .bind(&usuario)
        .fetch_one(&*pool)
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

// -----------------------------
// FUNCIONES DE INICIALIZACIÓN
// -----------------------------

#[tokio::main]
async fn main() {
    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();
    
    // Obtiene la URL de la base de datos desde las variables de entorno
    let db_url = dotenv!("DATABASE_URL"); // Aquí se usa la macro dotenvy_macro

    // Establece la conexión con la base de datos PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error al conectar con la base de datos");

    tauri::Builder::default()
        // Gestiona el pool de conexiones
        .manage(pool)
        // Registra los comandos
        .invoke_handler(tauri::generate_handler![
            login,
            empleados::create_employee,
            empleados::get_employees,
            empleados::update_employee,
            empleados::delete_employee,
            horarios::get_horarios,
            horarios::create_horario,
            horarios::update_horario,
            horarios::delete_horario,
        ])
        // Inicia la aplicación Tauri
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
