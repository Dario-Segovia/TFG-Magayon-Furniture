use serde::Serialize;
use tauri::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ComprasResumen {
    pub total_compras: i64,
    pub total_gastado: f64,
    pub proveedores_distintos: i64,
}

#[derive(Serialize)]
pub struct ComprasPorMes {
    pub mes: String,
    pub total: f64,
}

#[derive(Serialize)]
pub struct ProductoMasComprado {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Serialize)]
pub struct ProveedorTop {
    pub nombre: String,
    pub total_pagado: f64,
}

#[tauri::command]
pub async fn compras_resumen(pool: State<'_, PgPool>) -> Result<ComprasResumen, String> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as total_compras, COALESCE(SUM(total),0)::float8 as total_gastado, COUNT(DISTINCT id_proveedor) as proveedores_distintos FROM compras"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(ComprasResumen {
        total_compras: row.total_compras.unwrap_or(0),
        total_gastado: row.total_gastado.unwrap_or(0.0),
        proveedores_distintos: row.proveedores_distintos.unwrap_or(0),
    })
}

#[tauri::command]
pub async fn compras_por_mes(pool: State<'_, PgPool>) -> Result<Vec<ComprasPorMes>, String> {
    let rows = sqlx::query!(
        "SELECT TO_CHAR(fecha, 'YYYY-MM') as mes, COALESCE(SUM(total),0)::float8 as total
         FROM compras
         GROUP BY mes
         ORDER BY mes"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ComprasPorMes {
            mes: r.mes.unwrap_or_else(|| "Sin mes".to_string()),
            total: r.total.unwrap_or(0.0),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_mas_comprados(pool: State<'_, PgPool>) -> Result<Vec<ProductoMasComprado>, String> {
    let rows = sqlx::query!(
        "SELECT i.nombre, SUM(dc.cantidad) as cantidad
         FROM detalle_compras dc
         JOIN inventario i ON dc.id_producto = i.id
         GROUP BY i.nombre
         ORDER BY cantidad DESC
         LIMIT 5"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductoMasComprado {
            nombre: r.nombre,
            cantidad: r.cantidad.unwrap_or(0),
        })
        .collect())
}

#[tauri::command]
pub async fn proveedores_top(pool: State<'_, PgPool>) -> Result<Vec<ProveedorTop>, String> {
    let rows = sqlx::query!(
        "SELECT p.nombre, SUM(c.total)::float8 as total_pagado
         FROM compras c
         JOIN proveedores p ON c.id_proveedor = p.id
         GROUP BY p.nombre
         ORDER BY total_pagado DESC
         LIMIT 5"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProveedorTop {
            nombre: r.nombre,
            total_pagado: r.total_pagado.unwrap_or(0.0),
        })
        .collect())
}