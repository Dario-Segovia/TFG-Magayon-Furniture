use sqlx::{PgPool, postgres::PgRow};
use tauri::command;
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;
use std::env;
use sqlx::Row;
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
    pub fecha_contratacion: NaiveDate,  // Cambiar a NaiveDate en vez de String
}

// Cargar la configuración de la base de datos desde el archivo .env
pub async fn establish_connection() -> PgPool {
    dotenv().ok();  // Carga el archivo .env

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurado en .env");

    // Crear un pool de conexiones para PostgreSQL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");
    
    pool
}

// Función para intentar parsear una fecha de cadena a NaiveDate
fn parse_fecha_contratacion(fecha_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d")  // Asegura que esté en el formato "YYYY-MM-DD"
}
#[command]
pub async fn create_employee(
    nombre: String,
    apellido: String,
    email: String,
    telefono: Option<String>,
    puesto: String,
    salario: Option<f64>,
    fecha_contratacion: String,
) -> Result<String, String> {
    let pool = establish_connection().await;
    
    // Intentamos parsear la fecha antes de guardarla
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
        .execute(&pool)
        .await
    {
        Ok(_) => Ok("Empleado creado exitosamente.".to_string()),
        Err(e) => {
            // Imprimir el error detallado en consola para depuración
            println!("Error al crear el empleado: {:?}", e);
            Err(format!("Error al crear el empleado: {}", e))
        }
    }
}

// Obtener todos los empleados
#[command]
pub async fn get_employees() -> Result<Vec<Empleado>, String> {
    let pool = establish_connection().await;
    
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
            fecha_contratacion: row.get(7),  // Deberías almacenar el tipo NaiveDate aquí
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(rows)
}

// Actualizar un empleado
#[command]
pub async fn update_employee(
    id: i32,
    nombre: String,
    apellido: String,
    email: String,
    telefono: Option<String>,
    puesto: String,
    salario: Option<f64>,
    fecha_contratacion: String,  // Recibimos la fecha como String
) -> Result<String, String> {
    let pool = establish_connection().await;

    // Intentamos parsear la fecha antes de actualizar
    let fecha_contratacion = match parse_fecha_contratacion(&fecha_contratacion) {
        Ok(fecha) => fecha,
        Err(_) => return Err("Error al parsear la fecha de contratación.".to_string()),
    };

    let query = "
        UPDATE public.empleados SET nombre = $1, apellido = $2, email = $3, telefono = $4, puesto = $5, salario = $6, fecha_contratacion = $7
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
        .execute(&pool)
        .await
    {
        Ok(_) => Ok("Empleado actualizado exitosamente.".to_string()),
        Err(e) => {
            println!("Error al actualizar el empleado: {:?}", e);
            Err(format!("Error al actualizar el empleado: {}", e))
        }
    }
}
// Eliminar un empleado
#[command]
pub async fn delete_employee(id: i32) -> Result<String, String> {
    let pool = establish_connection().await;
    
    let query = "DELETE FROM public.empleados WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(_) => Ok("Empleado eliminado exitosamente.".to_string()),
        Err(e) => {
            // Imprimir el error detallado en consola para depuración
            println!("Error al eliminar el empleado: {:?}", e);
            Err(format!("Error al eliminar el empleado: {}", e))
        }
    }
}


