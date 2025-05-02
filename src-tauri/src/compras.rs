use tauri::State;
use sqlx::{PgPool, Row};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Compra {
    pub id: i32,
    pub fecha: Option<String>,
    pub id_proveedor: Option<i32>,
    pub total: Option<f64>,
}



#[derive(serde::Deserialize)]
pub struct CompraData {
    #[serde(alias = "idProveedor", alias = "id_proveedor")]
    id_proveedor: i32,
    total: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductoCompra {
    pub nombre: String,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompraConDetalles {
    pub id: i32,
    pub fecha: Option<String>,
    pub nombre_proveedor: String,
    pub total: Option<f64>,
    pub productos: Vec<ProductoCompra>,
}


#[derive(Serialize, Debug)]
pub struct CompraConProveedor {
    pub id: i32,
    pub fecha: Option<String>,
    pub id_proveedor: Option<i32>,
    pub nombre_proveedor: Option<String>,
    pub total: Option<f64>,
}

#[derive(serde::Deserialize)]

pub struct DetalleCompraData {
    pub id_compra: i32,
    
    pub cantidad: i32,
    pub precio_unitario: f64,

    // Nuevos campos para crear el producto en inventario si no existe
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: Option<String>,
}



#[tauri::command]
pub async fn crear_detalle_compra(
    pool: State<'_, PgPool>,
    data: DetalleCompraData,
) -> Result<(), String> {
    // Intentar encontrar el producto por nombre y categoría
    let producto_id: Option<i32> = sqlx::query_scalar(
        r#"
        SELECT id FROM inventario
        WHERE nombre = $1 AND categoria = $2
        "#,
    )
    .bind(&data.nombre)
    .bind(&data.categoria)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al buscar producto en inventario: {}", e))?;

    // Si no existe, insertarlo en inventario
    let final_id_producto = match producto_id {
        Some(id) => id,
        None => {
            let row = sqlx::query(
                r#"
                INSERT INTO inventario (nombre, descripcion, cantidad, precio_unitario, categoria)
                VALUES ($1, $2, 0, $3, $4)
                RETURNING id
                "#
            )
            .bind(&data.nombre)
            .bind(&data.descripcion)
            .bind(data.precio_unitario)
            .bind(&data.categoria)
            .fetch_one(&*pool)
            .await
            .map_err(|e| format!("Error al insertar nuevo producto en inventario: {}", e))?;
            row.get("id")
        }
    };

    // Insertar el detalle de la compra
    sqlx::query(
        r#"
        INSERT INTO detalle_compras (id_compra, id_producto, cantidad, precio_unitario)
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(data.id_compra)
    .bind(final_id_producto)
    .bind(data.cantidad)
    .bind(data.precio_unitario)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al insertar detalle de compra: {}", e))?;

    Ok(())
}




#[tauri::command]
pub async fn crear_compra(
    pool: State<'_, PgPool>,
    data: CompraData,  // Ahora recibimos una estructura
) -> Result<i32, String> {
    println!("Datos recibidos - id_proveedor: {}, total: {}", data.id_proveedor, data.total);

    // Validar que el proveedor existe
    let proveedor_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM proveedores WHERE id = $1)"
    )
    .bind(data.id_proveedor)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error validando proveedor: {}", e))?;

    if !proveedor_existe {
        return Err(format!("El proveedor con ID {} no existe", data.id_proveedor));
    }

    // Insertar la compra
    let row = sqlx::query(
        r#"
        INSERT INTO compras (id_proveedor, total)
        VALUES ($1, $2)
        RETURNING id
        "#
    )
    .bind(data.id_proveedor)
    .bind(data.total)
    .fetch_one(&*pool)
    .await
    .map_err(|e| format!("Error al crear compra: {}", e))?;

    Ok(row.get::<i32, _>("id"))
}


#[tauri::command]
pub async fn listar_compras_con_proveedor(pool: State<'_, PgPool>) -> Result<Vec<CompraConProveedor>, String> {
    let rows = sqlx::query(
        r#"
        SELECT 
            c.id,
            c.fecha::TEXT, 
            c.id_proveedor, 
            p.nombre as nombre_proveedor,
            c.total
        FROM compras c
        LEFT JOIN proveedores p ON c.id_proveedor = p.id
        ORDER BY c.fecha DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let compras: Vec<CompraConProveedor> = rows.into_iter().map(|row| {
        CompraConProveedor {
            id: row.get("id"),
            fecha: row.get("fecha"),
            id_proveedor: row.get("id_proveedor"),
            nombre_proveedor: row.get("nombre_proveedor"),
            total: row.get::<Option<Decimal>, _>("total").map(|d| d.to_f64().unwrap()),
        }
    }).collect();

    Ok(compras)
}





#[tauri::command]
pub async fn obtener_productos_compra(
    pool: State<'_, PgPool>,
    id_compra: i32,
) -> Result<Vec<ProductoCompra>, String> {
    let rows = sqlx::query(
        r#"
        SELECT 
            i.nombre,
            dc.cantidad,
            dc.precio_unitario
        FROM detalle_compras dc
        JOIN inventario i ON dc.id_producto = i.id
        WHERE dc.id_compra = $1
        "#
    )
    .bind(id_compra)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let productos = rows.into_iter().map(|row| {
        ProductoCompra {
            nombre: row.get("nombre"),
            cantidad: row.get("cantidad"),
            precio_unitario: row.get("precio_unitario"),
        }
    }).collect();

    Ok(productos)
}




#[tauri::command]
pub async fn actualizar_compra(
    pool: State<'_, PgPool>,
    id: i32,
    id_proveedor: Option<i32>,
    total: Option<f64>
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE compras 
        SET id_proveedor = $1, total = $2 
        WHERE id = $3
        "#
    )
    .bind(id_proveedor)
    .bind(total)
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn eliminar_compra(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    sqlx::query(
        r#"
        DELETE FROM compras 
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}


