/*!
 * Archivo: horarios.rs
 * Proyecto: Magayon Furniture
 * 
 * Descripción general:
 * --------------------
 * Este módulo gestiona las operaciones relacionadas con los horarios laborales de los empleados.
 * Permite crear, consultar, actualizar y eliminar registros de horarios, facilitando la organización y planificación de turnos.
 * 
 * Funcionalidades principales:
 * ----------------------------
 * 1. `get_horarios`:  
 *    Obtiene la lista completa de horarios registrados, con detalles como empleado, fecha, hora de inicio y fin, tipo de turno, etc.
 * 
 * 2. `create_horario`:  
 *    Permite crear un nuevo registro de horario para un empleado.
 * 
 * 3. `update_horario`:  
 *    Actualiza un horario existente con nuevos datos (horas, fechas, tipo de turno).
 * 
 * 4. `delete_horario`:  
 *    Elimina un registro de horario de la base de datos.
 * 
 * Dependencias esperadas:
 * -----------------------
 * - `sqlx` para la gestión de la base de datos PostgreSQL.
 * - `tauri::command` para exponer estas funciones al frontend de la aplicación.
 * 
 * Notas:
 * ------
 * - Es importante validar solapamientos o inconsistencias en los horarios antes de la inserción o actualización.
 * - Se espera que los horarios estén vinculados a los empleados existentes en la base de datos.
 */

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
    println!("Datos recibidos: {:?}", horario);

    sqlx::query(
        r#"
        INSERT INTO horarios (empleado_id, fecha, hora_inicio, hora_fin, tipo_turno, notas)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(horario.empleado_id)
    .bind(horario.fecha)
    .bind(horario.hora_inicio)
    .bind(horario.hora_fin)
    .bind(horario.tipo_turno)
    .bind(horario.notas)
    .execute(&*pool)
    .await
    .map_err(|e| format!("Error al crear horario: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_horario(pool: State<'_, PgPool>, id: i32, horario: NuevoHorario) -> Result<(), String> {
    println!("Datos recibidos para actualizar: id = {}, horario = {:?}", id, horario);

    sqlx::query(
        r#"
        UPDATE horarios
        SET empleado_id = $1, fecha = $2, hora_inicio = $3, hora_fin = $4, tipo_turno = $5, notas = $6
        WHERE id = $7
        "#,
    )
    .bind(horario.empleado_id)
    .bind(horario.fecha)
    .bind(horario.hora_inicio)
    .bind(horario.hora_fin)
    .bind(horario.tipo_turno)
    .bind(horario.notas)
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al actualizar horario: {}", e);
        format!("Error al actualizar horario: {}", e)
    })?;

    println!("Horario actualizado correctamente: id = {}", id);
    Ok(())
}

#[tauri::command]
pub async fn delete_horario(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM horarios WHERE id = $1")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al eliminar horario: {}", e))?;

    Ok(())
}