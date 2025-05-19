use serde::Serialize;
use tauri::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ProveedoresResumen {
    pub total: i64,
    pub activos: i64,
    pub con_contrato: i64,
    pub paises: i64,
}

#[derive(Serialize)]
pub struct ProveedoresPorPais {
    pub pais: String,
    pub total: i64,
}

#[derive(Serialize)]
pub struct ProductosPorProveedor {
    pub proveedor: String,
    pub total_productos: i64,
}

#[derive(Serialize)]
pub struct ProductoMasCaroProveedor {
    pub proveedor: String,
    pub producto: String,
    pub precio_unitario: f64,
}

#[tauri::command]
pub async fn proveedores_resumen(pool: State<'_, PgPool>) -> Result<ProveedoresResumen, String> {
    let row = sqlx::query!(
        "SELECT COUNT(*) as total,
                COUNT(*) FILTER (WHERE estado = 'activo') as activos,
                COUNT(*) FILTER (WHERE contrato_vigente = true) as con_contrato,
                COUNT(DISTINCT pais) as paises
         FROM proveedores"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(ProveedoresResumen {
        total: row.total.unwrap_or(0),
        activos: row.activos.unwrap_or(0),
        con_contrato: row.con_contrato.unwrap_or(0),
        paises: row.paises.unwrap_or(0),
    })
}

#[tauri::command]
pub async fn proveedores_por_pais(pool: State<'_, PgPool>) -> Result<Vec<ProveedoresPorPais>, String> {
    let rows = sqlx::query!(
        "SELECT pais, COUNT(*) as total FROM proveedores GROUP BY pais ORDER BY total DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProveedoresPorPais {
            pais: r.pais.unwrap_or_else(|| "Sin país".to_string()),
            total: r.total.unwrap_or(0),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_por_proveedor(pool: State<'_, PgPool>) -> Result<Vec<ProductosPorProveedor>, String> {
    let rows = sqlx::query!(
        "SELECT p.nombre as proveedor, COUNT(pp.id) as total_productos
         FROM proveedores p
         LEFT JOIN proveedor_productos pp ON p.id = pp.proveedor_id
         GROUP BY p.nombre
         ORDER BY total_productos DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductosPorProveedor {
            proveedor: r.proveedor,
            total_productos: r.total_productos.unwrap_or(0),
        })
        .collect())
}

#[tauri::command]
pub async fn productos_mas_caros_por_proveedor(pool: State<'_, PgPool>) -> Result<Vec<ProductoMasCaroProveedor>, String> {
    let rows = sqlx::query!(
        "SELECT p.nombre as proveedor, pp.producto, pp.precio_unitario::float8 as precio_unitario
         FROM proveedor_productos pp
         JOIN proveedores p ON pp.proveedor_id = p.id
         WHERE (pp.proveedor_id, pp.precio_unitario) IN (
            SELECT proveedor_id, MAX(precio_unitario)
            FROM proveedor_productos
            GROUP BY proveedor_id
         )
         ORDER BY precio_unitario DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows
        .into_iter()
        .map(|r| ProductoMasCaroProveedor {
            proveedor: r.proveedor,
            producto: r.producto,
            precio_unitario: r.precio_unitario.unwrap_or(0.0),
        })
        .collect())
}