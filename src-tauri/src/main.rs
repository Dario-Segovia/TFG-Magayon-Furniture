#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[warn(dead_code)]
use bcrypt::verify;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tauri::State;
use std::env;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

mod build;
mod clientes;
mod empleados;
mod horarios;
mod proveedores;
mod inventario;

// -----------------------------
// FUNCIONES DE LOGIN
// -----------------------------

#[tauri::command]
async fn login(
    usuario: String,
    password: String,
    pool: State<'_, PgPool>,
) -> Result<String, String> {
    // Consulta el usuario y la contraseña cifrada en la base de datos
    let row: (String, String) =
        sqlx::query_as("SELECT password, rol FROM public.usuarios WHERE usuario = $1")
            .bind(&usuario)
            .fetch_one(&*pool)
            .await
            .map_err(|e| format!("Usuario no encontrado o error en la consulta: {}", e))?;

    // Desencripta la contraseña y compara
    let (hashed_password, rol) = row;
    if verify(&password, &hashed_password)
        .map_err(|e| format!("Error al verificar la contraseña: {}", e))?
    {
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error al conectar con la base de datos");

    tauri::Builder::default()
        .manage(pool)
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
            clientes::crear_cliente,
            clientes::obtener_clientes,
            clientes::obtener_cliente_por_id,
            clientes::eliminar_cliente,
            clientes::actualizar_cliente,
            proveedores::crear_proveedor,
            proveedores::obtener_proveedores,
            proveedores::actualizar_proveedor,
            proveedores::eliminar_proveedor,
            proveedores::agregar_producto_proveedor,
            proveedores::obtener_productos_proveedor,
            proveedores::eliminar_producto_proveedor,
            inventario::get_inventory,
            inventario::get_inventory_item,
            inventario::add_inventory_item,
            inventario::update_inventory_item,
            inventario::delete_inventory_item,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}