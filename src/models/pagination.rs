use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    limit_field: Option<u64>,

    offset_field: Option<u64>,

    order_field: Option<String>,

    order_type_field: Option<String>,
}
