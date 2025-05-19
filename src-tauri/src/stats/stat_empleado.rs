use rust_decimal::prelude::ToPrimitive;
use serde::Serialize;
use tauri::State;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ConteoPorCategoria {
    pub categoria: String,
    pub total: i64,
}

#[derive(Serialize)]
pub struct PromedioPorCategoria {
    pub categoria: String,
    pub promedio: f64,
}

#[derive(Serialize)]
pub struct EmpleadoInfo {
    pub nombre: String,
    pub apellido: String,
    pub salario: f64,
}

#[derive(Serialize)]
pub struct PromedioAntiguedad {
    pub dias_promedio: f64,
}

#[tauri::command]
pub async fn empleados_totales(pool: State<'_, PgPool>) -> Result<i64, String> {
    let registro = sqlx::query!("SELECT COUNT(*) as total FROM empleados")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(registro.total.unwrap_or(0))
}

#[tauri::command]
pub async fn empleados_por_puesto(pool: State<'_, PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT puesto as categoria, COUNT(*) as total FROM empleados GROUP BY puesto ORDER BY total DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| ConteoPorCategoria {
            categoria: r.categoria,
            total: r.total.unwrap_or(0),
        })
        .collect();

    Ok(resultado)
}

#[tauri::command]
pub async fn salario_promedio_por_puesto(pool: State<'_, PgPool>) -> Result<Vec<PromedioPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT puesto as categoria, AVG(salario) as promedio FROM empleados GROUP BY puesto"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| PromedioPorCategoria {
            categoria: r.categoria,
            promedio: r.promedio.unwrap_or(0.0),
        })
        .collect();

    Ok(resultado)
}

#[tauri::command]
pub async fn salario_total(pool: State<'_, PgPool>) -> Result<f64, String> {
    let registro = sqlx::query!("SELECT SUM(salario) as total FROM empleados")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(registro.total.unwrap_or(0.0))
}

#[tauri::command]
pub async fn antiguedad_promedio(pool: State<'_, PgPool>) -> Result<f64, String> {
    let registro = sqlx::query!(
        "SELECT AVG((current_date - fecha_contratacion)) AS dias_promedio FROM empleados"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(registro
        .dias_promedio
        .map(|d| d.to_f64().unwrap_or(0.0))
        .unwrap_or(0.0))
}

#[tauri::command]
pub async fn contrataciones_por_anio(pool: State<'_, PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT EXTRACT(YEAR FROM fecha_contratacion)::TEXT as categoria, COUNT(*) as total FROM empleados GROUP BY categoria ORDER BY categoria"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| ConteoPorCategoria {
            categoria: r.categoria.unwrap_or_else(|| "Desconocido".into()),
            total: r.total.unwrap_or(0),
        })
        .collect();

    Ok(resultado)
}

#[tauri::command]
pub async fn empleado_mayor_salario(pool: State<'_, PgPool>) -> Result<EmpleadoInfo, String> {
    let registro = sqlx::query!(
        "SELECT nombre, apellido, salario FROM empleados ORDER BY salario DESC LIMIT 1"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(EmpleadoInfo {
        nombre: registro.nombre,
        apellido: registro.apellido,
        salario: registro.salario.unwrap_or(0.0),
    })
}

#[tauri::command]
pub async fn contrataciones_ultimos_meses(pool: State<'_, PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT TO_CHAR(fecha_contratacion, 'YYYY-MM') AS categoria, COUNT(*) as total FROM empleados WHERE fecha_contratacion >= CURRENT_DATE - INTERVAL '12 months' GROUP BY categoria ORDER BY categoria"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| ConteoPorCategoria {
            categoria: r.categoria.unwrap_or_else(|| "Desconocido".into()),
            total: r.total.unwrap_or(0),
        })
        .collect();

    Ok(resultado)
}
