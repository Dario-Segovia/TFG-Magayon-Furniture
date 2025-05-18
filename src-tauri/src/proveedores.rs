use rust_decimal::Decimal;
use tauri::State;
use sqlx::{PgPool, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Proveedor {
    pub id: Option<i32>, // Cambiado a Option<i32> para que sea opcional
    pub nombre: String,
    pub contacto: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub pais: Option<String>,
    pub estado: Option<String>,
    pub contrato_vigente: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ProductoProveedor {
    pub id: Option<i32>, 
    #[serde(alias = "proveedorId")] // Acepta proveedorId (camelCase) del frontend
    pub proveedor_id: i32,
    pub producto: String,
    pub descripcion: Option<String>,
    #[sqlx(try_from = "f64")]  // Allow conversion from FLOAT8
    pub precio_unitario: Decimal,  // Still use Decimal internally
    pub categoria: Option<String>,
}


#[tauri::command]
pub async fn crear_proveedor(
    pool: State<'_, PgPool>,
    proveedor: Proveedor,
) -> Result<(), String> {
    println!("Datos recibidos para crear proveedor: {:?}", proveedor);

    sqlx::query(
        r#"
        INSERT INTO proveedores (nombre, contacto, telefono, email, direccion, pais, estado, contrato_vigente)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&proveedor.nombre)
    .bind(&proveedor.contacto)
    .bind(&proveedor.telefono)
    .bind(&proveedor.email)
    .bind(&proveedor.direccion)
    .bind(&proveedor.pais)
    .bind(proveedor.estado.unwrap_or_else(|| "activo".to_string()))
    .bind(proveedor.contrato_vigente.unwrap_or(false))
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al crear proveedor: {}", e);
        format!("Error al crear proveedor: {}", e)
    })?;

    println!("Proveedor creado correctamente.");
    Ok(())
}

#[tauri::command]
pub async fn obtener_proveedores(pool: State<'_, PgPool>) -> Result<Vec<Proveedor>, String> {
    println!("Obteniendo lista de proveedores...");

    let proveedores = sqlx::query_as::<_, Proveedor>( 
        r#"
        SELECT id, nombre, contacto, telefono, email, direccion, pais, estado, contrato_vigente
        FROM proveedores
        ORDER BY nombre ASC
        "#,
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        println!("Error al obtener proveedores: {}", e);
        format!("Error al obtener proveedores: {}", e)
    })?;

    println!("Lista de proveedores obtenida correctamente. Total: {}", proveedores.len());
    Ok(proveedores)
}

#[tauri::command]
pub async fn actualizar_proveedor(
    pool: State<'_, PgPool>,
    id: i32,
    proveedor: Proveedor,
) -> Result<(), String> {
    println!("Datos recibidos para actualizar proveedor: id = {}, datos = {:?}", id, proveedor);

    sqlx::query(
        r#"
        UPDATE proveedores
        SET nombre = $1, contacto = $2, telefono = $3, email = $4, direccion = $5, pais = $6, estado = $7, contrato_vigente = $8
        WHERE id = $9
        "#,
    )
    .bind(&proveedor.nombre)
    .bind(&proveedor.contacto)
    .bind(&proveedor.telefono)
    .bind(&proveedor.email)
    .bind(&proveedor.direccion)
    .bind(&proveedor.pais)
    .bind(proveedor.estado.unwrap_or_else(|| "activo".to_string()))
    .bind(proveedor.contrato_vigente.unwrap_or(false))
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al actualizar proveedor: {}", e);
        format!("Error al actualizar proveedor: {}", e)
    })?;

    println!("Proveedor actualizado correctamente: id = {}", id);
    Ok(())
}

#[tauri::command]
pub async fn eliminar_proveedor(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    println!("Eliminando proveedor con id = {}", id);

    sqlx::query(
        r#"
        DELETE FROM proveedores
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al eliminar proveedor: {}", e);
        format!("Error al eliminar proveedor: {}", e)
    })?;

    println!("Proveedor eliminado correctamente: id = {}", id);
    Ok(())
}

#[tauri::command]
pub async fn agregar_producto_proveedor(
    pool: State<'_, PgPool>,
    producto: ProductoProveedor,
) -> Result<(), String> {
    println!("Datos recibidos para agregar producto a proveedor: {:?}", producto);

    // 1. Insertar en proveedor_productos
    sqlx::query(
        r#"
        INSERT INTO proveedor_productos 
        (proveedor_id, producto, descripcion, precio_unitario, categoria)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(producto.proveedor_id)
    .bind(&producto.producto)
    .bind(&producto.descripcion)
    .bind(producto.precio_unitario)
    .bind(&producto.categoria)
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al agregar producto al proveedor: {}", e);
        format!("Error al agregar producto al proveedor: {}", e)
    })?;

    // 2. Comprobar si existe en inventario
    let existe: Option<(i32,)> = sqlx::query_as(
        "SELECT id FROM inventario WHERE LOWER(nombre) = LOWER($1) LIMIT 1"
    )
    .bind(&producto.producto)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Error al buscar en inventario: {}", e))?;

    if existe.is_none() {
        // 3. Si no existe, crear en inventario con cantidad 0
        sqlx::query(
            "INSERT INTO inventario (nombre, descripcion, cantidad, precio_unitario, categoria) VALUES ($1, $2, 0, $3, $4)"
        )
        .bind(&producto.producto)
        .bind(&producto.descripcion)
        .bind(producto.precio_unitario)
        .bind(&producto.categoria)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Error al crear producto en inventario: {}", e))?;
        println!("Producto creado en inventario automáticamente.");
    } else {
        println!("Producto ya existe en inventario, no se crea duplicado.");
    }

    println!("Producto agregado correctamente al proveedor: id proveedor = {}", producto.proveedor_id);
    Ok(())
}


#[tauri::command]
pub async fn obtener_productos_proveedor(
    pool: State<'_, PgPool>,
    proveedor_id: i32,
) -> Result<Vec<ProductoProveedor>, String> {
    let productos = sqlx::query_as::<_, ProductoProveedor>(
        r#"
        SELECT 
            id, 
            proveedor_id, 
            producto, 
            descripcion, 
            precio_unitario::FLOAT8 as precio_unitario,  -- Explicit cast to FLOAT8
            categoria
        FROM proveedor_productos
        WHERE proveedor_id = $1
        ORDER BY producto ASC
        "#,
    )
    .bind(proveedor_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("Error al obtener productos: {}", e))?;

    Ok(productos)
}

#[tauri::command]
pub async fn eliminar_producto_proveedor(pool: State<'_, PgPool>, id: i32) -> Result<(), String> {
    println!("Eliminando producto con id = {}", id);

    sqlx::query(
        r#"
        DELETE FROM proveedor_productos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&*pool)
    .await
    .map_err(|e| {
        println!("Error al eliminar producto del proveedor: {}", e);
        format!("Error al eliminar producto del proveedor: {}", e)
    })?;

    println!("Producto eliminado correctamente: id = {}", id);
    Ok(())
}
