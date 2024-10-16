use serde::{Deserialize, Serialize};

pub const QUERY_SET_METHOD: &str = "query/set";

#[derive(Debug, Serialize, Clone)]
pub struct QueryRequest {
    pub query: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct QueryResponse {}
