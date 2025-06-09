/*!
 * Archivo: inventario.rs
 * Proyecto: Magayon Furniture
 * 
 * Descripción general:
 * --------------------
 * Este módulo gestiona todas las operaciones relacionadas con el **inventario de productos** de la empresa.
 * Incluye funciones para obtener la lista de productos, añadir nuevos ítems, actualizar detalles y eliminar productos del inventario.
 * Además, se encarga de la lógica para importar inventario desde archivos XML.
 * 
 * Funcionalidades principales:
 * ----------------------------
 * 1. `get_inventory`:  
 *    Recupera la lista completa de productos en el inventario, con sus detalles principales (nombre, cantidad, categoría, precio, etc).
 * 
 * 2. `get_inventory_item`:  
 *    Obtiene los detalles completos de un producto específico dado su ID.
 * 
 * 3. `add_inventory_item`:  
 *    Añade un nuevo producto al inventario, incluyendo información como nombre, categoría, cantidad inicial y precio.
 * 
 * 4. `update_inventory_item`:  
 *    Actualiza los datos de un producto existente en el inventario.
 * 
 * 5. `delete_inventory_item`:  
 *    Elimina un producto del inventario.
 * 
 * 6. `importar_inventario_xml`:  
 *    Permite importar productos al inventario desde un archivo XML, facilitando la carga masiva de datos.
 * 
 * Dependencias esperadas:
 * -----------------------
 * - `sqlx` para la conexión y ejecución de consultas en PostgreSQL.
 * - `serde` y/o `quick-xml` para procesar archivos XML (en la función de importación).
 * - `tauri::command` para exponer las funciones al frontend.
 * 
 * Notas:
 * ------
 * - Las operaciones de actualización y eliminación deberían validar la existencia previa del producto.
 * - La importación XML debe manejar errores de formato y evitar duplicados según la lógica del proyecto.
 */
use sqlx::{PgPool, Row, postgres::PgRow};
use tauri::State;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use quick_xml::de::from_str;
use serde::de::{self, Deserializer};


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

#[derive(Deserialize, Debug)]
pub struct InventoryItemInput {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub cantidad: i32,
    #[serde(alias = "precioUnitario", deserialize_with = "deserialize_decimal_from_str")]
    pub precio_unitario: Decimal,
    pub categoria: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct InventarioXml {
    #[serde(rename = "item")]
    pub items: Vec<InventoryItemInput>,
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

#[tauri::command]
pub async fn importar_inventario_xml(
    xml_data: String,
    pool: State<'_, PgPool>,
) -> Result<(), String> {
    println!("XML recibido: {}", xml_data);

    // Prueba directa de deserialización
    match from_str::<InventarioXml>(&xml_data) {
        Ok(inventario) => {
            println!("Deserialización exitosa: {:?}", inventario);
            for item in inventario.items {
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
            }
            Ok(())
        }
        Err(e) => {
            println!("Error de deserialización: {:?}", e);
            Err(format!("Error de deserialización: {}", e))
        }
    }
}

fn deserialize_decimal_from_str<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Decimal::from_str_exact(&s).map_err(de::Error::custom)
}

