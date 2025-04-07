use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};
use sqlx::{FromRow, PgPool};
use tauri::State;

// -----------------------------
// MODELOS
// -----------------------------

#[derive(Serialize, FromRow)]
pub struct Horario {
    pub id: i32,
    pub empleado_id: i32,
    pub fecha: NaiveDate,
    pub hora_inicio: NaiveTime,
    pub hora_fin: NaiveTime,
    pub tipo_turno: Option<String>,
    pub notas: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NuevoHorario {
    #[serde(rename = "empleado_id")]
    pub empleado_id: i32,  // This is now part of the struct
    pub fecha: NaiveDate,
    pub hora_inicio: NaiveTime,
    pub hora_fin: NaiveTime,
    pub tipo_turno: String,
    pub notas: Option<String>,
}

// -----------------------------
// FUNCIONES CRUD
// -----------------------------

#[tauri::command]
pub async fn get_horarios(pool: State<'_, PgPool>) -> Result<Vec<Horario>, String> {
    sqlx::query_as::<_, Horario>("SELECT * FROM horarios ORDER BY fecha, hora_inicio")
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Error al obtener horarios: {}", e))
}

#[tauri::command]
pub async fn create_horario(pool: State<'_, PgPool>, horario: NuevoHorario) -> Result<(), String> {
    println!("Datos recibidos: {:?}", horario); // 👈 Añade esto para depurar

    sqlx::query!(
        r#"
        INSERT INTO horarios (empleado_id, fecha, hora_inicio, hora_fin, tipo_turno, notas)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        horario.empleado_id, // Asegúrate de que esto se está recibiendo
        horario.fecha,
        horario.hora_inicio,
        horario.hora_fin,
        horario.tipo_turno,
        horario.notas
    )
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear horario: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_horario(pool: State<'_, PgPool>, id: i32, horario: NuevoHorario) -> Result<(), String> {
    sqlx::query!(
        r#"
        UPDATE horarios
        SET empleado_id = $1, fecha = $2, hora_inicio = $3, hora_fin = $4, tipo_turno = $5, notas = $6
        WHERE id = $7
        "#,
        horario.empleado_id,
        horario.fecha,
        horario.hora_inicio,
        horario.hora_fin,
        horario.tipo_turno,
        horario.notas,
        id
    )
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al actualizar horario: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_horario(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    sqlx::query!("DELETE FROM horarios WHERE id = $1", id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar horario: {}", e))?;

    Ok(())
}
