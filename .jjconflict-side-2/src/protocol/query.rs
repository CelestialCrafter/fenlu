use serde::{Deserialize, Serialize};

pub const QUERY_METHOD: &str = "query/query";

#[derive(Debug, Serialize, Clone)]
pub struct QueryRequest {
    pub query: String
}
#[derive(Debug, Deserialize, Clone)]
pub struct QueryResponse {}
