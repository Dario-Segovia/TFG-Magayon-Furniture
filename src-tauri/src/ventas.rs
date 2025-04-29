use tauri::State;
use sqlx::{PgPool, Row};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Venta {
    pub id: i32,
    pub fecha: Option<String>,
    pub id_cliente: Option<i32>,
    pub total: Option<f64>,
}

#[tauri::command]
pub async fn crear_venta(
    pool: State<'_, PgPool>,
    id_cliente: Option<i32>,
    total: Option<f64>
) -> Result<i32, String> {
    let row = sqlx::query(
        r#"
        INSERT INTO ventas (id_cliente, total)
        VALUES ($1, $2)
        RETURNING id
        "#
    )
    .bind(id_cliente)
    .bind(total)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row.get("id"))
}

#[tauri::command]
pub async fn listar_ventas(pool: State<'_, PgPool>) -> Result<Vec<Venta>, String> {
    let rows = sqlx::query(
        r#"
        SELECT id, fecha::TEXT, id_cliente, total
        FROM ventas
        ORDER BY fecha DESC
        "#
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let ventas = rows.into_iter().map(|row| {
        Venta {
            id: row.get("id"),
            fecha: row.get("fecha"),
            id_cliente: row.get("id_cliente"),
            total: row
                .get::<Option<Decimal>, _>("total")
                .map(|d| d.to_f64().unwrap()),
        }
    }).collect();

    Ok(ventas)
}

#[tauri::command]
pub async fn actualizar_venta(
    pool: State<'_, PgPool>,
    id: i32,
    id_cliente: Option<i32>,
    total: Option<f64>
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE ventas
        SET id_cliente = $1, total = $2
        WHERE id = $3
        "#
    )
    .bind(id_cliente)
    .bind(total)
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn eliminar_venta(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    sqlx::query(
        r#"
        DELETE FROM ventas
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
