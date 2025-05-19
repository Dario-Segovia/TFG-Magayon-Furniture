use serde::Serialize;
use tauri::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct InventarioResumen {
    pub total_productos: i64,
    pub stock_total: i64,
    pub valor_total: f64,
}

#[derive(Serialize)]
pub struct StockPorCategoria {
    pub categoria: String,
    pub total: i64,
}

#[derive(Serialize)]
pub struct ProductoCritico {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Serialize)]
pub struct ProductoMasCaro {
    pub nombre: String,
    pub precio_unitario: f64,
}

#[tauri::command]
pub async fn inventario_resumen(pool: State<'_, PgPool>) -> Result<InventarioResumen, String> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as total_productos, COALESCE(SUM(cantidad),0) as stock_total, COALESCE(SUM(cantidad * precio_unitario),0)::float8 as valor_total FROM inventario"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(InventarioResumen {
        total_productos: row.total_productos.unwrap_or(0),
        stock_total: row.stock_total.unwrap_or(0),
        valor_total: row.valor_total.unwrap_or(0.0),
    })
}

#[tauri::command]
pub async fn stock_por_categoria(pool: State<'_, PgPool>) -> Result<Vec<StockPorCategoria>, String> {
    let rows = sqlx::query!(
        "SELECT categoria, SUM(cantidad) as total FROM inventario GROUP BY categoria ORDER BY total DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| StockPorCategoria {
            categoria: r.categoria.unwrap_or_else(|| "Sin categoría".to_string()),
            total: r.total.unwrap_or(0),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_criticos(pool: State<'_, PgPool>) -> Result<Vec<ProductoCritico>, String> {
    let rows = sqlx::query!(
        "SELECT nombre, cantidad FROM inventario WHERE cantidad < 10 ORDER BY cantidad ASC LIMIT 10"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductoCritico {
            nombre: r.nombre,
            cantidad: r.cantidad.into(),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_mas_caros(pool: State<'_, PgPool>) -> Result<Vec<ProductoMasCaro>, String> {
    let rows = sqlx::query!(
        "SELECT nombre, precio_unitario::float8 as precio_unitario FROM inventario ORDER BY precio_unitario DESC LIMIT 5"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductoMasCaro {
            nombre: r.nombre,
            precio_unitario: r.precio_unitario.unwrap_or(0.0),
        })
        .collect())
}