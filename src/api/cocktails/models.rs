use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListByFilterRequest{
    ids: Vec<uuid::Uuid>
}
