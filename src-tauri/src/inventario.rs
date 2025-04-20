use sqlx::{PgPool, Row, postgres::PgRow};
use tauri::State;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug)]
pub struct InventarioItem {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub cantidad: i32,
    #[serde(alias = "precioUnitario")]
    pub precio_unitario: Decimal,
    pub categoria: Option<String>,
}

#[derive(Deserialize)]
pub struct InventoryItemInput {
    nombre: String,
    descripcion: Option<String>,
    cantidad: i32,
    #[serde(alias = "precioUnitario")]
    precio_unitario: Decimal,
    categoria: Option<String>,
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
    item: InventoryItemInput,
    pool: State<'_, PgPool>,
) -> Result<(), String> {
    let query = "INSERT INTO public.inventario (nombre, descripcion, cantidad, precio_unitario, categoria) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(item.nombre)
        .bind(item.descripcion)
        .bind(item.cantidad)
        .bind(item.precio_unitario)
        .bind(item.categoria)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]

pub async fn update_inventory_item(
    id: i32,
    item: InventoryItemInput,  // Usamos la misma estructura que para add
    pool: State<'_, PgPool>,
) -> Result<(), String> {
    let query = "UPDATE public.inventario SET nombre = $1, descripcion = $2, cantidad = $3, precio_unitario = $4, categoria = $5 WHERE id = $6";

    sqlx::query(query)
        .bind(item.nombre)
        .bind(item.descripcion)
        .bind(item.cantidad)
        .bind(item.precio_unitario)
        .bind(item.categoria)
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