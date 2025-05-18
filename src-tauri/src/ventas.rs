
use tauri::State;
use sqlx::{PgPool, Row};
use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Venta {
    pub id: i32,
    pub fecha: NaiveDateTime,
    pub id_cliente: Option<i32>,
    pub nombre_cliente: Option<String>,
    pub total: Decimal,
    pub detalles: Vec<DetalleVenta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetalleVenta {
    pub id: i32,
    #[serde(rename = "idVenta")]
    pub id_venta: i32,
    pub id_producto: i32,
    pub nombre_producto: Option<String>,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct NuevaVenta {
    pub id_cliente: Option<i32>,
    pub detalles: Vec<NuevoDetalleVenta>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoDetalleVenta {
    pub id_producto: i32,
    pub cantidad: i32,
    pub precio_unitario: Decimal,
}

// Nuevo struct para manejar los parámetros de update_venta
#[derive(Debug, Deserialize)]
pub struct UpdateVentaParams {
    #[serde(rename = "idVenta")]
    pub id_venta: i32,
    pub data: NuevaVenta,
}

#[tauri::command]
pub async fn crear_venta(pool: State<'_, PgPool>, data: NuevaVenta) -> Result<i32, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let total: Decimal = data.detalles.iter()
        .map(|d| d.precio_unitario * Decimal::from(d.cantidad))
        .sum();

    let venta_id_row = sqlx::query("INSERT INTO ventas (id_cliente, total) VALUES ($1, $2) RETURNING id")
        .bind(data.id_cliente)
        .bind(total)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let venta_id: i32 = venta_id_row.get(0);

    for detalle in data.detalles {
        sqlx::query("INSERT INTO detalle_ventas (id_venta, id_producto, cantidad, precio_unitario) VALUES ($1, $2, $3, $4)")
            .bind(venta_id)
            .bind(detalle.id_producto)
            .bind(detalle.cantidad)
            .bind(detalle.precio_unitario)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(venta_id)
}

#[tauri::command]
pub async fn get_ventas(pool: State<'_, PgPool>) -> Result<Vec<Venta>, String> {
    let rows = sqlx::query("SELECT * FROM ventas")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let mut ventas = Vec::new();

    for row in rows {
        let id: i32 = row.get("id");
        let id_cliente: Option<i32> = row.get("id_cliente");

        let nombre_cliente = match id_cliente {
            Some(cliente_id) => {
                let query = sqlx::query("SELECT nombre FROM clientes WHERE id = $1")
                    .bind(cliente_id)
                    .fetch_optional(&*pool)
                    .await
                    .map_err(|e| e.to_string())?;
                query.map(|row| row.get("nombre"))
            }
            None => None,
        };

        let detalles = sqlx::query("SELECT * FROM detalle_ventas WHERE id_venta = $1")
            .bind(id)
            .fetch_all(&*pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut detalles_venta = Vec::new();
        for detalle_row in detalles {
            let id_producto: i32 = detalle_row.get("id_producto");

            let nombre_producto = sqlx::query("SELECT nombre FROM inventario WHERE id = $1")
                .bind(id_producto)
                .fetch_optional(&*pool)
                .await
                .map_err(|e| e.to_string())?
                .map(|row| row.get("nombre"));

            detalles_venta.push(DetalleVenta {
                id: detalle_row.get("id"),
                id_venta: detalle_row.get("id_venta"),
                id_producto,
                nombre_producto,
                cantidad: detalle_row.get("cantidad"),
                precio_unitario: detalle_row.get("precio_unitario"),
            });
        }

        ventas.push(Venta {
            id,
            fecha: row.get("fecha"),
            id_cliente,
            nombre_cliente,
            total: row.get("total"),
            detalles: detalles_venta,
        });
    }

    Ok(ventas)
}

#[tauri::command]
pub async fn update_venta(
    pool: State<'_, PgPool>, 
    params: UpdateVentaParams  // Ahora acepta el struct con el rename
) -> Result<(), String> {
    let UpdateVentaParams { id_venta, data } = params;
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let total: Decimal = data.detalles.iter()
        .map(|d| d.precio_unitario * Decimal::from(d.cantidad))
        .sum();

    sqlx::query("UPDATE ventas SET id_cliente = $1, total = $2 WHERE id = $3")
        .bind(data.id_cliente)
        .bind(total)
        .bind(id_venta)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM detalle_ventas WHERE id_venta = $1")
        .bind(id_venta)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    for detalle in data.detalles {
        sqlx::query("INSERT INTO detalle_ventas (id_venta, id_producto, cantidad, precio_unitario) VALUES ($1, $2, $3, $4)")
            .bind(id_venta)
            .bind(detalle.id_producto)
            .bind(detalle.cantidad)
            .bind(detalle.precio_unitario)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_venta(pool: State<'_, PgPool>, id_venta: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM ventas WHERE id = $1")
        .bind(id_venta)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}