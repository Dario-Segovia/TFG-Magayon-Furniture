use serde::Serialize;
use tauri::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct VentasResumen {
    pub total_ventas: i64,
    pub total_ingresos: f64,
    pub clientes_distintos: i64,
}

#[derive(Serialize)]
pub struct VentasPorMes {
    pub mes: String,
    pub total: f64,
}

#[derive(Serialize)]
pub struct ProductoMasVendido {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Serialize)]
pub struct ClienteTop {
    pub nombre: String,
    pub total_gastado: f64,
}

#[tauri::command]
pub async fn ventas_resumen(pool: State<'_, PgPool>) -> Result<VentasResumen, String> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as total_ventas, COALESCE(SUM(total),0)::float8 as total_ingresos, COUNT(DISTINCT id_cliente) as clientes_distintos FROM ventas"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(VentasResumen {
        total_ventas: row.total_ventas.unwrap_or(0),
        total_ingresos: row.total_ingresos.unwrap_or(0.0),
        clientes_distintos: row.clientes_distintos.unwrap_or(0),
    })
}

#[tauri::command]
pub async fn ventas_por_mes(pool: State<'_, PgPool>) -> Result<Vec<VentasPorMes>, String> {
    let rows = sqlx::query!(
        "SELECT TO_CHAR(fecha, 'YYYY-MM') as mes, COALESCE(SUM(total),0)::float8 as total
         FROM ventas
         GROUP BY mes
         ORDER BY mes"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| VentasPorMes {
            mes: r.mes.unwrap_or_else(|| "Sin mes".to_string()),
            total: r.total.unwrap_or(0.0),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_mas_vendidos(pool: State<'_, PgPool>) -> Result<Vec<ProductoMasVendido>, String> {
    let rows = sqlx::query!(
        "SELECT i.nombre, SUM(dv.cantidad) as cantidad
         FROM detalle_ventas dv
         JOIN inventario i ON dv.id_producto = i.id
         GROUP BY i.nombre
         ORDER BY cantidad DESC
         LIMIT 5"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductoMasVendido {
            nombre: r.nombre,
            cantidad: r.cantidad.unwrap_or(0),
        })
        .collect())
}

#[tauri::command]
pub async fn clientes_top(pool: State<'_, PgPool>) -> Result<Vec<ClienteTop>, String> {
    let rows = sqlx::query!(
        "SELECT c.nombre, SUM(v.total)::float8 as total_gastado
         FROM ventas v
         JOIN clientes c ON v.id_cliente = c.id
         GROUP BY c.nombre
         ORDER BY total_gastado DESC
         LIMIT 5"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ClienteTop {
            nombre: r.nombre,
            total_gastado: r.total_gastado.unwrap_or(0.0),
        })
        .collect())
}