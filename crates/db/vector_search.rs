use crate::queries::prompts;
use crate::History;
use crate::TokioPostgresError;
use crate::Transaction;

pub struct RelatedContext {
    pub chunk_id: i32,
    pub chunk_text: String,
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
pub async fn get_related_context(
    transaction: &Transaction<'_>,
    prompt_id: i32,
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<RelatedContext>, TokioPostgresError> {
    // Which datasets does the prompt use
    let datasets = prompts::prompt_datasets()
        .bind(transaction, &prompt_id)
        .all()
        .await?;
    // We just need the id's
    let datasets: Vec<i32> = datasets.iter().map(|dataset| dataset.dataset_id).collect();

    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

    // Find sections of documents that are related to the users question
    let related_context = transaction
        .query(
            "
                    SELECT 
                        id,
                        text 
                    FROM 
                        chunks
                    WHERE
                        document_id IN (
                            SELECT id FROM documents WHERE dataset_id = ANY($1)
                        )
                    ORDER BY 
                        embeddings <-> $2 
                    LIMIT $3;
                    ",
            &[&datasets, &embedding_data, &(limit as i64)],
        )
        .await?;

    // Just get the text from the returned rows
    let related_context: Vec<RelatedContext> = related_context
        .into_iter()
        .map(|content| RelatedContext {
            chunk_id: content.get(0),
            chunk_text: content.get(1),
        })
        .collect();

    Ok(related_context)
}

// Query the vector database using a similarity search.
// The prompt decides how we use the datasets
pub async fn search_history(
    transaction: &Transaction<'_>,
    user_id: i32,
    limit: i32,
    embeddings: Vec<f32>,
) -> Result<Vec<History>, TokioPostgresError> {
    // Format the embeddings in PGVector format
    let embedding_data = pgvector::Vector::from(embeddings.clone());

    // Find sections of documents that are related to the users question
    let responses = transaction
        .query(
            "
                SELECT 
                    conv.id::bigint,
                    LEFT(c.response, 255),
                    trim(both '\"' from to_json(c.created_at)::text) as created_at_iso,
                    c.created_at
                FROM 
                    chats c
                LEFT JOIN
                    conversations conv
                ON conv.id = c.conversation_id
                WHERE
                    conv.user_id = $1
                ORDER BY 
                    request_embeddings <-> $2 
                LIMIT $3;
            ",
            &[&user_id, &embedding_data, &(limit as i64)],
        )
        .await?;

    // Just get the text from the returned rows
    let results: Vec<History> = responses
        .into_iter()
        .map(|content| History {
            id: content.get(0),
            summary: content.get(1),
            created_at_iso: content.get(2),
            created_at: content.get(3),
        })
        .collect();

    Ok(results)
}
