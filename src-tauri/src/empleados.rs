use sqlx::{PgPool, postgres::PgRow, Row};
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, ParseError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Empleado {
    pub id: i32,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub telefono: Option<String>,
    pub puesto: String,
    pub salario: Option<f64>,
    pub fecha_contratacion: NaiveDate,
}

fn parse_fecha_contratacion(fecha_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d")
}

#[command]
pub async fn create_employee(
    pool: State<'_, PgPool>,
    nombre: String,
    apellido: String,
    email: String,
    telefono: Option<String>,
    puesto: String,
    salario: Option<f64>,
    fecha_contratacion: String,
) -> Result<String, String> {
    let fecha_contratacion = match NaiveDate::parse_from_str(&fecha_contratacion, "%Y-%m-%d") {
        Ok(fecha) => fecha,
        Err(_) => return Err("La fecha de contratación debe estar en formato YYYY-MM-DD.".to_string()),
    };

    let query = "
        INSERT INTO public.empleados (nombre, apellido, email, telefono, puesto, salario, fecha_contratacion)
        VALUES ($1, $2, $3, $4, $5, $6, $7)";

    match sqlx::query(query)
        .bind(nombre)
        .bind(apellido)
        .bind(email)
        .bind(telefono)
        .bind(puesto)
        .bind(salario)
        .bind(fecha_contratacion)
        .execute(&*pool)
        .await
    {
        Ok(_) => Ok("Empleado creado exitosamente.".to_string()),
        Err(e) => {
            println!("Error al crear el empleado: {:?}", e);
            Err(format!("Error al crear el empleado: {}", e))
        }
    }
}

#[command]
pub async fn get_employees(pool: State<'_, PgPool>) -> Result<Vec<Empleado>, String> {
    let query = "SELECT id, nombre, apellido, email, telefono, puesto, salario, fecha_contratacion FROM public.empleados";

    let rows = sqlx::query(query)
        .map(|row: PgRow| Empleado {
            id: row.get(0),
            nombre: row.get(1),
            apellido: row.get(2),
            email: row.get(3),
            telefono: row.get(4),
            puesto: row.get(5),
            salario: row.get(6),
            fecha_contratacion: row.get(7),
        })
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[command]
pub async fn update_employee(
    pool: State<'_, PgPool>,
    id: i32,
    nombre: String,
    apellido: String,
    email: String,
    telefono: Option<String>,
    puesto: String,
    salario: Option<f64>,
    fecha_contratacion: String,
) -> Result<String, String> {
    let fecha_contratacion = match parse_fecha_contratacion(&fecha_contratacion) {
        Ok(fecha) => fecha,
        Err(_) => return Err("Error al parsear la fecha de contratación.".to_string()),
    };

    let query = "
        UPDATE public.empleados 
        SET nombre = $1, apellido = $2, email = $3, telefono = $4, puesto = $5, salario = $6, fecha_contratacion = $7
        WHERE id = $8";

    match sqlx::query(query)
        .bind(nombre)
        .bind(apellido)
        .bind(email)
        .bind(telefono)
        .bind(puesto)
        .bind(salario)
        .bind(fecha_contratacion)
        .bind(id)
        .execute(&*pool)
        .await
    {
        Ok(_) => Ok("Empleado actualizado exitosamente.".to_string()),
        Err(e) => {
            println!("Error al actualizar el empleado: {:?}", e);
            Err(format!("Error al actualizar el empleado: {}", e))
        }
    }
}

#[command]
pub async fn delete_employee(pool: State<'_, PgPool>, id: i32) -> Result<String, String> {
    let query = "DELETE FROM public.empleados WHERE id = $1";

    match sqlx::query(query)
        .bind(id)
        .execute(&*pool)
        .await
    {
        Ok(_) => Ok("Empleado eliminado exitosamente.".to_string()),
        Err(e) => {
            println!("Error al eliminar el empleado: {:?}", e);
            Err(format!("Error al eliminar el empleado: {}", e))
        }
    }
}
