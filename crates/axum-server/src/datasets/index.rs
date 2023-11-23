use crate::authentication::Authentication;
use crate::errors::CustomError;
use axum::extract::{Extension, Path};
use axum::response::Html;
use db::queries::{datasets, models};
use db::{ModelType, Pool};

pub async fn index(
    Path(organisation_id): Path<i32>,
    current_user: Authentication,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    super::super::rls::set_row_level_security_user(&transaction, &current_user).await?;

    let datasets = datasets::datasets().bind(&transaction).all().await?;

    let models = models::models()
        .bind(&transaction, &ModelType::Embeddings)
        .all()
        .await?;

    Ok(Html(ui_components::datasets::index(
        ui_components::datasets::index::PageProps {
            organisation_id,
            datasets,
            models,
        },
    )))
}
