use serde::{Serialize, Deserialize};
use tauri::State;
use tokio_postgres::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Compra {
    pub id: Option<i32>,
    pub fecha: Option<String>,
    pub id_proveedor: i32,
    pub estado: Option<String>,
    pub total: Option<f64>,
}

#[tauri::command]
pub async fn listar_compras(db: State<'_, Client>) -> Result<Vec<Compra>, String> {
    let db = db.inner();
    let rows = db.query("SELECT id, fecha, id_proveedor, estado, total FROM ordenes WHERE tipo = 'proveedor'", &[])
        .await
        .map_err(|e| e.to_string())?;
    
    let compras = rows.into_iter().map(|row| Compra {
        id: Some(row.get(0)),
        fecha: Some(row.get(1)),
        id_proveedor: row.get(2),
        estado: Some(row.get(3)),
        total: Some(row.get(4)),
    }).collect();
    Ok(compras)
}

#[tauri::command]
pub async fn crear_compra(db: State<'_, Client>, compra: Compra) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "INSERT INTO ordenes (tipo, id_proveedor, estado, total) VALUES ('proveedor', $1, $2, $3)",
        &[&compra.id_proveedor, &compra.estado, &compra.total]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub async fn actualizar_compra(db: State<'_, Client>, compra: Compra) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "UPDATE ordenes SET id_proveedor = $1, estado = $2, total = $3 WHERE id = $4 AND tipo = 'proveedor'",
        &[&compra.id_proveedor, &compra.estado, &compra.total, &compra.id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn eliminar_compra(db: State<'_, Client>, id: i32) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "DELETE FROM ordenes WHERE id = $1 AND tipo = 'proveedor'",
        &[&id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}