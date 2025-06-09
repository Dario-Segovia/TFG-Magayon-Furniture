/*!
 * Archivo: main.rs
 * Proyecto: Magayon Furniture
 * 
 * Descripción general:
 * --------------------
 * Este archivo constituye el punto de entrada principal de la aplicación Tauri escrita en Rust.
 * Se encarga de:
 * 
 * 1. **Inicializar la aplicación**:
 *    - Carga las variables de entorno (`.env`).
 *    - Conecta con la base de datos PostgreSQL mediante `sqlx`.
 *    - Configura el plugin de sistema de archivos (`tauri_plugin_fs`).
 * 
 * 2. **Gestión de estado y comandos**:
 *    - Administra el estado de conexión (`PgPool`) mediante `State`.
 *    - Expone múltiples comandos a la interfaz frontend mediante `#[tauri::command]`, agrupados por módulos funcionales como empleados, clientes, proveedores, inventario, ventas, compras y estadísticas.
 * 
 * 3. **Módulos incluidos**:
 *    - `empleados`: Gestión de CRUD de empleados.
 *    - `horarios`: Gestión de horarios de empleados.
 *    - `clientes`: Gestión de clientes.
 *    - `proveedores`: CRUD de proveedores y productos asociados.
 *    - `inventario`: CRUD del inventario.
 *    - `ventas`: Registro y gestión de ventas.
 *    - `compras`: Registro y gestión de compras.
 *    - `estadisticas`: Cálculo de métricas generales.
 *    - `stats`: Submódulo para estadísticas específicas por entidad (clientes, empleados, etc).
 * 
 * 4. **Login seguro**:
 *    - La función `login` valida las credenciales de usuario consultando la base de datos y verificando contraseñas cifradas con `bcrypt`.
 * 
 * 5. **Ejecución**:
 *    - Usa `tokio::main` para habilitar funciones asincrónicas.
 *    - Construye y ejecuta la aplicación Tauri con todos los comandos registrados.
 * 
 * Dependencias principales:
 * - `tauri`, `tauri_plugin_fs`
 * - `dotenvy`, `dotenvy_macro`
 * - `sqlx`
 * - `bcrypt`
 * 
 * Notas:
 * -------
 * Este archivo actúa como un **orquestador de módulos y comandos**, manteniendo el núcleo de configuración de la aplicación. Toda la lógica de negocio está separada en módulos para mantener un diseño modular y limpio.
 */


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[warn(dead_code)]
use bcrypt::verify;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tauri::State;
use tauri_plugin_fs;

mod build;
mod clientes;
mod compras;
mod empleados;

mod horarios;
mod inventario;
mod proveedores;
mod ventas;
mod estadisticas;  // Declara el módulo "estadisticas" (busca estadisticas/mod.rs)
mod stats;

// Para usar EstatEmpleados:
use stats::stat_cliente;
use stats::stat_empleado;
use stats::stat_horario;
use stats::stat_inventario;
use stats::stat_proveedores;
use stats::stat_ventas;
use stats::stat_compras;

// -----------------------------
// FUNCIONES DE LOGIN
// -----------------------------

#[tauri::command]
async fn login(
    usuario: String,
    password: String,
    pool: State<'_, PgPool>,
) -> Result<String, String> {
    // Consulta el usuario y la contraseña cifrada en la base de datos
    let row: (String, String) =
        sqlx::query_as("SELECT password, rol FROM public.usuarios WHERE usuario = $1")
            .bind(&usuario)
            .fetch_one(&*pool)
            .await
            .map_err(|e| format!("Usuario no encontrado o error en la consulta: {}", e))?;

    // Desencripta la contraseña y compara
    let (hashed_password, rol) = row;
    if verify(&password, &hashed_password)
        .map_err(|e| format!("Error al verificar la contraseña: {}", e))?
    {
        Ok(rol) // Retorna el rol si la contraseña es correcta
    } else {
        Err("Credenciales incorrectas".to_string()) // Si la contraseña es incorrecta
    }
}

// -----------------------------
// FUNCIONES DE INICIALIZACIÓN
// -----------------------------

#[tokio::main]

async fn main() {
    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();

    // Obtiene la URL de la base de datos desde las variables de entorno
    let db_url = dotenv!("DATABASE_URL"); // Aquí se usa la macro dotenvy_macro

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error al conectar con la base de datos");

    
tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .manage(pool)
    .invoke_handler(tauri::generate_handler![
        login,
        empleados::create_employee,
        empleados::get_employees,
        empleados::update_employee,
        empleados::delete_employee,
        horarios::get_horarios,
        horarios::create_horario,
        horarios::update_horario,
        horarios::delete_horario,
        clientes::crear_cliente,
        clientes::obtener_clientes,
        clientes::obtener_cliente_por_id,
        clientes::eliminar_cliente,
        clientes::actualizar_cliente,
        proveedores::crear_proveedor,
        proveedores::obtener_proveedores,
        proveedores::actualizar_proveedor,
        proveedores::eliminar_proveedor,
        proveedores::agregar_producto_proveedor,
        proveedores::obtener_productos_proveedor,
        proveedores::eliminar_producto_proveedor,
        inventario::get_inventory,
        inventario::get_inventory_item,
        inventario::add_inventory_item,
        inventario::update_inventory_item,
        inventario::delete_inventory_item,
        ventas::crear_venta,
        ventas::get_ventas,
        ventas::update_venta,
        ventas::delete_venta,
        compras::crear_compra,
        compras::actualizar_compra,
        compras::listar_compras_con_proveedor,
        compras::obtener_productos_compra,
        compras::eliminar_compra,
        compras::buscar_producto_inventario,
        compras::crear_detalle_compra,
        compras::obtener_detalles_compra,
        compras::listar_proveedores,
        estadisticas::get_ventas_por_mes,
        estadisticas::get_top_productos,
        estadisticas::get_stock_critico,
        estadisticas::get_ingresos_por_categoria,
        estadisticas::get_ventas_y_clientes,
        stat_cliente::obtener_total_clientes,
        stat_cliente::clientes_por_provincia,
        stat_cliente::clientes_por_ciudad,
        stat_cliente::clientes_por_pais,
        stat_empleado::empleados_totales,
        stat_empleado::empleados_por_puesto,
        stat_empleado::salario_promedio_por_puesto,
        stat_empleado::salario_total,
        stat_empleado::antiguedad_promedio,
        stat_empleado::empleado_mayor_salario,
        stat_empleado::contrataciones_ultimos_meses,
        stat_empleado::contrataciones_por_anio,
        stat_horario::resumen_horarios,
        stat_horario::horas_por_empleado,
        stat_horario::turnos_por_tipo,
        stat_horario::turnos_por_empleado,
        stat_inventario::inventario_resumen,
        stat_inventario::stock_por_categoria,
        stat_inventario::productos_criticos,
        stat_inventario::productos_mas_caros,
        stat_proveedores::proveedores_resumen,
        stat_proveedores::proveedores_por_pais,
        stat_proveedores::productos_por_proveedor,
        stat_proveedores::productos_mas_caros_por_proveedor,
        stat_ventas::ventas_resumen,
        stat_ventas::ventas_por_mes,
        stat_ventas::productos_mas_vendidos,
        stat_ventas::clientes_top,
        stat_compras::compras_resumen,
        stat_compras::compras_por_mes,
        stat_compras::productos_mas_comprados,
        stat_compras::proveedores_top,
        inventario::importar_inventario_xml,
    ])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application");
}
