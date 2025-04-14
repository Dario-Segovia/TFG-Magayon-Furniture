use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cliente {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
}

#[tauri::command]
pub async fn obtener_clientes(pool: State<'_, PgPool>) -> Result<Vec<Cliente>, String> {
    sqlx::query_as::<_, Cliente>(
        "SELECT id, nombre, email, telefono, direccion FROM public.clientes ORDER BY id"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener clientes: {}", e))
}

#[tauri::command]
pub async fn obtener_cliente_por_id(id: i32, pool: State<'_, PgPool>) -> Result<Cliente, String> {
    sqlx::query_as::<_, Cliente>(
        "SELECT id, nombre, email, telefono, direccion FROM public.clientes WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Cliente no encontrado: {}", e))
}

#[tauri::command]
pub async fn crear_cliente(
    nombre: String,
    email: String,
    telefono: Option<String>,
    direccion: Option<String>,
    pool: State<'_, PgPool>
) -> Result<String, String> {
    sqlx::query(
        "INSERT INTO public.clientes (nombre, email, telefono, direccion) VALUES ($1, $2, $3, $4)"
    )
    .bind(nombre)
    .bind(email)
    .bind(telefono)
    .bind(direccion)
    .execute(&*pool)
    .await
    .map(|_| "Cliente creado exitosamente".to_string())
    .map_err(|e| format!("Error al crear cliente: {}", e))
}

#[tauri::command]
pub async fn actualizar_cliente(
    id: i32,
    nombre: String,
    email: String,
    telefono: Option<String>,
    direccion: Option<String>,
    pool: State<'_, PgPool>
) -> Result<String, String> {
    sqlx::query(
        "UPDATE public.clientes SET nombre = $1, email = $2, telefono = $3, direccion = $4 WHERE id = $5"
    )
    .bind(nombre)
    .bind(email)
    .bind(telefono)
    .bind(direccion)
    .bind(id)
    .execute(&*pool)
    .await
    .map(|res| {
        if res.rows_affected() == 0 {
            Err("Cliente no encontrado".to_string())
        } else {
            Ok("Cliente actualizado exitosamente".to_string())
        }
    })
    .map_err(|e| format!("Error al actualizar cliente: {}", e))?
}

#[tauri::command]
pub async fn eliminar_cliente(id: i32, pool: State<'_, PgPool>) -> Result<String, String> {
    sqlx::query("DELETE FROM public.clientes WHERE id = $1")
        .bind(id)
        .execute(&*pool)
        .await
        .map(|res| {
            if res.rows_affected() == 0 {
                Err("Cliente no encontrado".to_string())
            } else {
                Ok("Cliente eliminado exitosamente".to_string())
            }
        })
        .map_err(|e| format!("Error al eliminar cliente: {}", e))?
}