use tauri::State;
use sqlx::{PgPool, Row};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime; 

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


#[derive(Debug, Serialize, Deserialize)]
pub struct CompraConProveedorYDetalles {
    pub id: i32,
    pub fecha: String,
    pub id_proveedor: Option<i32>,
    pub nombre_proveedor: Option<String>,
    pub total: Option<f64>,
    pub productos: Vec<DetalleCompra>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct DetalleCompra {
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: f64,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CompraDetalle {
    pub id_compra: i32,
    pub fecha: String,
    pub id_proveedor: i32,
    pub total: f64,
    pub id_producto: i32,
    pub nombre_producto: String,
    pub cantidad: i32,
    pub precio_unitario: f64,
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
pub async fn listar_compras_con_proveedor(pool: State<'_, PgPool>) -> Result<Vec<CompraConProveedorYDetalles>, String> {
    // Primero obtenemos todas las compras con su proveedor
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
        "#)
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut compras: Vec<CompraConProveedorYDetalles> = Vec::new();

    for row in rows {
        let id_compra: i32 = row.get("id");

        // Segundo: obtener productos de esta compra (detalles)
        let detalles = sqlx::query(
            r#"
            SELECT id_producto, cantidad, precio_unitario
            FROM detalle_compras
            WHERE id_compra = $1
            "#)
            .bind(id_compra)
            .fetch_all(&*pool)
            .await
            .map_err(|e| e.to_string())?;

        let productos: Vec<DetalleCompra> = detalles
            .into_iter()
            .map(|detalle| DetalleCompra {
                id_producto: detalle.get("id_producto"),
                cantidad: detalle.get("cantidad"),
                precio_unitario: detalle.get::<Decimal, _>("precio_unitario").to_f64().unwrap(),
            })
            .collect();

        // Agregar la compra con su proveedor y detalles
        compras.push(CompraConProveedorYDetalles {
            id: id_compra,
            fecha: row.get("fecha"),
            id_proveedor: row.get("id_proveedor"),
            nombre_proveedor: row.get("nombre_proveedor"),
            total: row.get::<Option<Decimal>, _>("total").map(|d| d.to_f64().unwrap()),
            productos, // Aquí incluimos los productos asociados a esta compra
        });
    }

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

#[tauri::command]
pub async fn actualizar_compra(
    pool: State<'_, PgPool>, 
    id: i32,
    id_proveedor: i32,
    total: f64,
    fecha: String,
) -> Result<(), String> {
    // Convertir la fecha a NaiveDateTime usando el formato adecuado
    let fecha_convertida = NaiveDateTime::parse_from_str(&fecha, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| format!("Error al parsear la fecha: {}", e))?;

    // Actualizar la compra en la base de datos
    sqlx::query(
        r#"
        UPDATE compras
        SET id_proveedor = $1, total = $2, fecha = $3
        WHERE id = $4
        "#,
    )
    .bind(id_proveedor)
    .bind(total)
    .bind(fecha_convertida) // Usar la fecha convertida
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}


#[tauri::command]
pub async fn obtener_detalles_compra(
    pool: State<'_, PgPool>,
    id_compra: i32,
) -> Result<Vec<CompraDetalle>, String> {
    // Realizar el JOIN entre las tablas compras, detalle_compras e inventario
    let rows = sqlx::query(
        r#"
        SELECT 
            c.id AS id_compra,
            c.fecha,
            c.id_proveedor,
            c.total,
            dc.id_producto,
            i.nombre AS nombre_producto,
            dc.cantidad,
            dc.precio_unitario
        FROM compras c
        JOIN detalle_compras dc ON c.id = dc.id_compra
        JOIN inventario i ON dc.id_producto = i.id
        WHERE c.id = $1
        "#,
    )
    .bind(id_compra)
    .map(|row: sqlx::postgres::PgRow| {
        CompraDetalle {
            id_compra: row.get("id_compra"),
            fecha: row.get("fecha"),
            id_proveedor: row.get("id_proveedor"),
            total: row.get("total"),
            id_producto: row.get("id_producto"),
            nombre_producto: row.get("nombre_producto"),
            cantidad: row.get("cantidad"),
            precio_unitario: row.get("precio_unitario"),
        }
    })
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error obteniendo detalles de la compra: {}", e))?;

    Ok(rows)
}