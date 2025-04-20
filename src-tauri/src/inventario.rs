use sqlx::{PgPool, Row, postgres::PgRow};
use tauri::State;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal; // ← CAMBIADO aquí
// Removed incorrect import for Decimal

#[derive(Serialize, Deserialize, Debug)]
pub struct InventarioItem {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub cantidad: i32,
    pub precio_unitario: Decimal, // ← CAMBIADO aquí
    pub categoria: Option<String>,
}

#[tauri::command]
pub async fn get_inventory(pool: State<'_, PgPool>) -> Result<Vec<InventarioItem>, String> {
    let query = "SELECT id, nombre, descripcion, cantidad, precio_unitario, categoria FROM public.inventario";

    let rows = sqlx::query(query)
        .map(|row: PgRow| InventarioItem {
            id: row.get(0),
            nombre: row.get(1),
            descripcion: row.get(2),
            cantidad: row.get(3),
            precio_unitario: row.get(4),
            categoria: row.get(5),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub async fn get_inventory_item(id: i32, pool: State<'_, PgPool>) -> Result<InventarioItem, String> {
    let query = "SELECT id, nombre, descripcion, cantidad, precio_unitario, categoria FROM public.inventario WHERE id = $1";

    let row = sqlx::query(query)
        .bind(id)
        .map(|row: PgRow| InventarioItem {
            id: row.get(0),
            nombre: row.get(1),
            descripcion: row.get(2),
            cantidad: row.get(3),
            precio_unitario: row.get(4),
            categoria: row.get(5),
        })
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(row)
}

#[tauri::command]
pub async fn add_inventory_item(
    nombre: String,
    descripcion: Option<String>,
    cantidad: i32,
    precio_unitario: Decimal,
    categoria: Option<String>,
    pool: State<'_, PgPool>,
) -> Result<(), String> {
    let query = "INSERT INTO public.inventario (nombre, descripcion, cantidad, precio_unitario, categoria) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(nombre)
        .bind(descripcion)
        .bind(cantidad)
        .bind(precio_unitario) // No conversion needed
        .bind(categoria)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_inventory_item(
    id: i32,
    nombre: String,
    descripcion: Option<String>,
    cantidad: i32,
    precio_unitario: Decimal,
    categoria: Option<String>,
    pool: State<'_, PgPool>,
) -> Result<(), String> {
    let query = "UPDATE public.inventario SET nombre = $1, descripcion = $2, cantidad = $3, precio_unitario = $4, categoria = $5 WHERE id = $6";

    sqlx::query(query)
        .bind(nombre)
        .bind(descripcion)
        .bind(cantidad)
        .bind(precio_unitario) // ← CONVERSIÓN necesaria
        .bind(categoria)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_inventory_item(id: i32, pool: State<'_, PgPool>) -> Result<(), String> {
    let query = "DELETE FROM public.inventario WHERE id = $1";

    sqlx::query(query)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
