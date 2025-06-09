/*!
 * Archivo: estadisticas.rs
 * Proyecto: Magayon Furniture
 * 
 * Descripción general:
 * --------------------
 * Este módulo está dedicado a la obtención y procesamiento de datos estadísticos relacionados con las operaciones
 * del sistema. Proporciona funciones que generan informes y métricas útiles para el análisis del negocio.
 * 
 * Funcionalidades principales:
 * ----------------------------
 * - Obtención de ventas por mes, permitiendo analizar tendencias temporales.
 * - Cálculo de los productos más vendidos para identificar artículos clave.
 * - Identificación del stock crítico para gestionar inventarios y evitar faltantes.
 * - Cálculo de ingresos por categorías para evaluar el rendimiento de diferentes líneas de productos.
 * - Análisis combinado de ventas y clientes para estudiar el comportamiento y fidelidad.
 * 
 * Dependencias esperadas:
 * -----------------------
 * - Uso de consultas SQL optimizadas para extracción eficiente de datos.
 * - Funciones expuestas como comandos Tauri para ser consumidas desde el frontend.
 * 
 * Notas:
 * ------
 * - Se recomienda ejecutar estas funciones en contextos asíncronos para no bloquear la aplicación.
 * - Los resultados se suelen presentar en forma de listas o agregados para facilitar su uso en visualizaciones.
 */



use sqlx::{PgPool, Row, postgres::PgRow};
use tauri::State;

// ---------- ESTRUCTURAS ----------

#[derive(Debug, serde::Serialize)]
pub struct VentasPorMes {
    pub mes: String,
    pub total_ventas: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct ProductoMasVendido {
    pub nombre: String,
    pub total_vendidos: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct ProductoCritico {
    pub id: i32,
    pub nombre: String,
    pub cantidad: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct IngresoPorCategoria {
    pub categoria: String,
    pub ingresos: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct VentasYClientesPorMes {
    pub mes: String,
    pub clientes_unicos: i64,
    pub total_ventas: f64,
}

// ---------- FUNCIONES ----------

#[tauri::command]
pub async fn get_ventas_por_mes(pool: State<'_, PgPool>) -> Result<Vec<VentasPorMes>, String> {
    let query = r#"
        SELECT TO_CHAR(fecha, 'YYYY-MM') AS mes, SUM(total)::float8 AS total_ventas
        FROM ventas
        GROUP BY mes
        ORDER BY mes;
    "#;

    let rows = sqlx::query(query)
        .map(|row: PgRow| VentasPorMes {
            mes: row.get("mes"),
            total_ventas: row.get("total_ventas"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub async fn get_top_productos(pool: State<'_, PgPool>) -> Result<Vec<ProductoMasVendido>, String> {
    let query = r#"
        SELECT i.nombre, SUM(dv.cantidad) AS total_vendidos
        FROM detalle_ventas dv
        JOIN inventario i ON dv.id_producto = i.id
        GROUP BY i.nombre
        ORDER BY total_vendidos DESC
        LIMIT 5;
    "#;

    let rows = sqlx::query(query)
        .map(|row: PgRow| ProductoMasVendido {
            nombre: row.get("nombre"),
            total_vendidos: row.get("total_vendidos"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub async fn get_stock_critico(pool: State<'_, PgPool>) -> Result<Vec<ProductoCritico>, String> {
    let query = r#"
        SELECT id, nombre, cantidad
        FROM inventario
        WHERE cantidad < 10
        ORDER BY cantidad ASC;
    "#;

    let rows = sqlx::query(query)
        .map(|row: PgRow| ProductoCritico {
            id: row.get("id"),
            nombre: row.get("nombre"),
            cantidad: row.get("cantidad"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub async fn get_ingresos_por_categoria(pool: State<'_, PgPool>) -> Result<Vec<IngresoPorCategoria>, String> {
    let query = r#"
        SELECT i.categoria, SUM(dv.cantidad * dv.precio_unitario)::float8 AS ingresos
        FROM detalle_ventas dv
        JOIN inventario i ON dv.id_producto = i.id
        GROUP BY i.categoria
        ORDER BY ingresos DESC;
    "#;

    let rows = sqlx::query(query)
        .map(|row: PgRow| IngresoPorCategoria {
            categoria: row.get("categoria"),
            ingresos: row.get("ingresos"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[tauri::command]
pub async fn get_ventas_y_clientes(pool: State<'_, PgPool>) -> Result<Vec<VentasYClientesPorMes>, String> {
    let query = r#"
        SELECT TO_CHAR(v.fecha, 'YYYY-MM') AS mes,
               COUNT(DISTINCT v.id_cliente) AS clientes_unicos,
               SUM(v.total)::float8 AS total_ventas
        FROM ventas v
        GROUP BY mes
        ORDER BY mes;
    "#;

    let rows = sqlx::query(query)
        .map(|row: PgRow| VentasYClientesPorMes {
            mes: row.get("mes"),
            clientes_unicos: row.get("clientes_unicos"),
            total_ventas: row.get("total_ventas"),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}
