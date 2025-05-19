// Cliente básico (opcional si se necesita en otras estadísticas)
#[derive(Debug, serde::Serialize)]
pub struct Cliente {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub telefono: Option<String>,
    pub via: Option<String>,
    pub numero: Option<String>,
    pub ciudad: Option<String>,
    pub provincia: Option<String>,
    pub pais: Option<String>,
}

// Para total de clientes
#[derive(Debug, serde::Serialize)]
pub struct TotalClientes {
    pub total: i64,
}

// Para conteos por agrupación
#[derive(Debug, serde::Serialize)]
pub struct ConteoPorCategoria {
    pub categoria: String, // ciudad / provincia / pais
    pub total: i64,
}



#[tauri::command]
pub async fn obtener_total_clientes(pool: tauri::State<'_, sqlx::PgPool>) -> Result<TotalClientes, String> {
    let row = sqlx::query!("SELECT COUNT(*) as total FROM clientes")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(TotalClientes { total: row.total.unwrap_or(0) })
}



#[tauri::command]
pub async fn clientes_por_provincia(pool: tauri::State<'_, sqlx::PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT provincia as categoria, COUNT(*) as total FROM clientes GROUP BY provincia ORDER BY total DESC"
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
pub async fn clientes_por_ciudad(pool: tauri::State<'_, sqlx::PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT ciudad as categoria, COUNT(*) as total FROM clientes GROUP BY ciudad ORDER BY total DESC"
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
pub async fn clientes_por_pais(pool: tauri::State<'_, sqlx::PgPool>) -> Result<Vec<ConteoPorCategoria>, String> {
    let registros = sqlx::query!(
        "SELECT pais as categoria, COUNT(*) as total FROM clientes GROUP BY pais ORDER BY total DESC"
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

