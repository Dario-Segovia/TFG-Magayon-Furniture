use serde::Serialize;
use tauri::State;
use sqlx::PgPool;
use num_traits::ToPrimitive;

#[derive(Serialize)]
pub struct ConteoPorCategoria {
    pub categoria: String,
    pub total: i64,
}

#[derive(Serialize)]
pub struct HorasPorEmpleadoConNombre {
    pub nombre: String,
    pub total_horas: f64,
}

#[derive(Serialize)]
pub struct TurnosPorEmpleadoConNombre {
    pub nombre: String,
    pub tipo_turno: String,
    pub cantidad: i64,
}

#[derive(Serialize)]
pub struct HorarioResumen {
    pub total_registros: i64,
    pub total_horas: f64,
    pub empleados_distintos: i64,
}

#[tauri::command]
pub async fn resumen_horarios(pool: State<'_, PgPool>) -> Result<HorarioResumen, String> {
    let total_registros = sqlx::query!("SELECT COUNT(*) as total FROM horarios")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?
        .total
        .unwrap_or(0);

    let empleados_distintos = sqlx::query!("SELECT COUNT(DISTINCT empleado_id) as total FROM horarios")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?
        .total
        .unwrap_or(0);

    use rust_decimal::Decimal;

    let total_horas_decimal = sqlx::query!(
        "SELECT COALESCE(SUM(EXTRACT(EPOCH FROM (hora_fin - hora_inicio))/3600),0) as total_horas FROM horarios"
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?
    .total_horas
    .unwrap_or(Decimal::ZERO);

    let total_horas = total_horas_decimal.to_f64().unwrap_or(0.0);

    Ok(HorarioResumen {
        total_registros,
        total_horas,
        empleados_distintos,
    })
}

#[tauri::command]
pub async fn horas_por_empleado(pool: State<'_, PgPool>) -> Result<Vec<HorasPorEmpleadoConNombre>, String> {
    let registros = sqlx::query!(
        "
        SELECT e.nombre || ' ' || e.apellido AS nombre,
               SUM(EXTRACT(EPOCH FROM (h.hora_fin - h.hora_inicio))/3600) as total_horas
        FROM horarios h
        JOIN empleados e ON h.empleado_id = e.id
        GROUP BY e.nombre, e.apellido
        ORDER BY total_horas DESC
        "
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| HorasPorEmpleadoConNombre {
            nombre: r.nombre.unwrap_or_else(|| "Desconocido".into()),
            total_horas: r.total_horas.and_then(|h| h.to_f64()).unwrap_or(0.0),
        })
        .collect();

    Ok(resultado)
}

#[tauri::command]
pub async fn turnos_por_tipo(pool: State<'_, PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT tipo_turno as categoria, COUNT(*) as total FROM horarios GROUP BY tipo_turno ORDER BY total DESC"
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
pub async fn turnos_por_empleado(pool: State<'_, PgPool>) -> Result<Vec<TurnosPorEmpleadoConNombre>, String> {
    let registros = sqlx::query!(
        "
        SELECT e.nombre || ' ' || e.apellido AS nombre,
               h.tipo_turno,
               COUNT(*) as cantidad
        FROM horarios h
        JOIN empleados e ON h.empleado_id = e.id
        GROUP BY e.nombre, e.apellido, h.tipo_turno
        ORDER BY nombre, cantidad DESC
        "
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let resultado = registros
        .into_iter()
        .map(|r| TurnosPorEmpleadoConNombre {
            nombre: r.nombre.unwrap_or_else(|| "Desconocido".into()),
            tipo_turno: r.tipo_turno.unwrap_or_else(|| "Desconocido".into()),
            cantidad: r.cantidad.unwrap_or(0),
        })
        .collect();

    Ok(resultado)
}
