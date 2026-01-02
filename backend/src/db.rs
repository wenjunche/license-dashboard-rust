use crate::models::*;
use sqlx::PgPool;

pub async fn list_app_configs(
    pool: &PgPool,
    query: &AppConfigQuery,
) -> Result<AppConfigListResponse, sqlx::Error> {
    let offset = (query.page - 1) * query.per_page;
    
    // Build WHERE clause dynamically
    let mut conditions = Vec::new();
    let mut bind_count = 1;
    
    if query.contract.is_some() {
        conditions.push(format!("contract = ${}", bind_count));
        bind_count += 1;
    }
    
    if query.url.is_some() {
        match query.url_match {
            MatchType::Exact => {
                conditions.push(format!("url = ${}", bind_count));
            }
            MatchType::Fuzzy => {
                conditions.push(format!("url ILIKE ${}", bind_count));
            }
        }
        bind_count += 1;
    }
    
    if query.uuid.is_some() {
        match query.uuid_match {
            MatchType::Exact => {
                conditions.push(format!("uuid = ${}", bind_count));
            }
            MatchType::Fuzzy => {
                conditions.push(format!("uuid ILIKE ${}", bind_count));
            }
        }
        bind_count += 1;
    }
    
    if query.app_name.is_some() {
        match query.app_name_match {
            MatchType::Exact => {
                conditions.push(format!("name = ${}", bind_count));
            }
            MatchType::Fuzzy => {
                conditions.push(format!("name ILIKE ${}", bind_count));
            }
        }
        bind_count += 1;
    }
    
    if let Some(ref billable_filter) = query.billable {
        match billable_filter {
            BillableFilter::All => {},
            BillableFilter::Review => {
                conditions.push("billable IS NULL".to_string());
            }
            BillableFilter::True => {
                conditions.push("billable = true".to_string());
            }
            BillableFilter::False => {
                conditions.push("billable = false".to_string());
            }
        }
    }
    
    if let Some(ignore) = query.ignore_files {
        conditions.push(format!("ignore_files = ${}", bind_count));
        bind_count += 1;
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    // Build ORDER BY clause
    let sort_column = query.sort_by.as_deref().unwrap_or("id");
    let sort_direction = match query.sort_order {
        SortOrder::Asc => "ASC",
        SortOrder::Desc => "DESC",
    };
    let order_clause = format!("ORDER BY {} {}", sort_column, sort_direction);
    
    // Count total
    let count_query = format!("SELECT COUNT(*) as count FROM app_configs {}", where_clause);
    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(pool)
        .await?;
    
    // Fetch data
    let data_query = format!(
        "SELECT * FROM app_configs {} {} LIMIT ${} OFFSET ${}",
        where_clause, order_clause, bind_count, bind_count + 1
    );
    
    let data: Vec<AppConfig> = sqlx::query_as(&data_query)
        .bind(query.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;
    
    let total_pages = ((total as f64) / (query.per_page as f64)).ceil() as u32;
    
    Ok(AppConfigListResponse {
        data,
        total,
        page: query.page,
        per_page: query.per_page,
        total_pages,
    })
}

pub async fn create_app_config(
    pool: &PgPool,
    input: CreateAppConfig,
) -> Result<AppConfig, sqlx::Error> {
    let config = sqlx::query_as::<_, AppConfig>(
        r#"
        INSERT INTO app_configs (contract, url, uuid, name, billable, ignore_files)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(&input.contract)
    .bind(&input.url)
    .bind(&input.uuid)
    .bind(&input.name)
    .bind(input.billable)
    .bind(input.ignore_files)
    .fetch_one(pool)
    .await?;
    
    Ok(config)
}

pub async fn get_app_config_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<AppConfig>, sqlx::Error> {
    let config = sqlx::query_as::<_, AppConfig>(
        "SELECT * FROM app_configs WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(config)
}

pub async fn update_app_config(
    pool: &PgPool,
    id: i32,
    input: UpdateAppConfig,
) -> Result<Option<AppConfig>, sqlx::Error> {
    let config = sqlx::query_as::<_, AppConfig>(
        r#"
        UPDATE app_configs
        SET
            contract = COALESCE($1, contract),
            url = COALESCE($2, url),
            uuid = COALESCE($3, uuid),
            name = COALESCE($4, name),
            billable = COALESCE($5, billable),
            ignore_files = COALESCE($6, ignore_files),
            updated_at = NOW()
        WHERE id = $7
        RETURNING *
        "#
    )
    .bind(input.contract)
    .bind(input.url)
    .bind(input.uuid)
    .bind(input.name)
    .bind(input.billable)
    .bind(input.ignore_files)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(config)
}

pub async fn delete_app_config(
    pool: &PgPool,
    id: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM app_configs WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn bulk_update_app_configs(
    pool: &PgPool,
    ids: &[i32],
    input: &UpdateAppConfig,
) -> Result<Vec<AppConfig>, sqlx::Error> {
    let configs = sqlx::query_as::<_, AppConfig>(
        r#"
        UPDATE app_configs
        SET
            contract = COALESCE($1, contract),
            url = COALESCE($2, url),
            uuid = COALESCE($3, uuid),
            name = COALESCE($4, name),
            billable = COALESCE($5, billable),
            ignore_files = COALESCE($6, ignore_files),
            updated_at = NOW()
        WHERE id = ANY($7)
        RETURNING *
        "#
    )
    .bind(&input.contract)
    .bind(&input.url)
    .bind(&input.uuid)
    .bind(&input.name)
    .bind(input.billable)
    .bind(input.ignore_files)
    .bind(ids)
    .fetch_all(pool)
    .await?;
    
    Ok(configs)
}
