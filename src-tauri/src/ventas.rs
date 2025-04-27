use serde::{Serialize, Deserialize};
use tauri::State;
use tokio_postgres::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Venta {
    pub id: Option<i32>,
    pub fecha: Option<String>,
    pub id_cliente: i32,
    pub estado: Option<String>,
    pub total: Option<f64>,
}

#[tauri::command]
pub async fn listar_ventas(db: State<'_, Client>) -> Result<Vec<Venta>, String> {
    let db = db.inner();
    let rows = db.query("SELECT id, fecha, id_cliente, estado, total FROM ordenes WHERE tipo = 'cliente'", &[])
        .await
        .map_err(|e| e.to_string())?;
    
    let ventas = rows.into_iter().map(|row| Venta {
        id: Some(row.get(0)),
        fecha: Some(row.get(1)),
        id_cliente: row.get(2),
        estado: Some(row.get(3)),
        total: Some(row.get(4)),
    }).collect();
    Ok(ventas)
}

#[tauri::command]
pub async fn crear_venta(db: State<'_, Client>, venta: Venta) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "INSERT INTO ordenes (tipo, id_cliente, estado, total) VALUES ('cliente', $1, $2, $3)",
        &[&venta.id_cliente, &venta.estado, &venta.total]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn actualizar_venta(db: State<'_, Client>, venta: Venta) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "UPDATE ordenes SET id_cliente = $1, estado = $2, total = $3 WHERE id = $4 AND tipo = 'cliente'",
        &[&venta.id_cliente, &venta.estado, &venta.total, &venta.id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn eliminar_venta(db: State<'_, Client>, id: i32) -> Result<(), String> {
    let db = db.inner();
    db.execute(
        "DELETE FROM ordenes WHERE id = $1 AND tipo = 'cliente'",
        &[&id]
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}