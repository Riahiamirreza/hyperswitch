use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::{enums, schema::routing_algorithm};

#[derive(Clone, Debug, Identifiable, Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = routing_algorithm, primary_key(algorithm_id), check_for_backend(diesel::pg::Pg))]
pub struct RoutingAlgorithm {
    pub algorithm_id: String,
    pub profile_id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub name: String,
    pub description: Option<String>,
    pub kind: enums::RoutingAlgorithmKind,
    pub algorithm_data: serde_json::Value,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub algorithm_for: enums::TransactionType,
}

pub struct RoutingAlgorithmMetadata {
    pub algorithm_id: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: enums::RoutingAlgorithmKind,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub algorithm_for: enums::TransactionType,
}

pub struct RoutingProfileMetadata {
    pub profile_id: common_utils::id_type::ProfileId,
    pub algorithm_id: String,
    pub name: String,
    pub description: Option<String>,
    pub kind: enums::RoutingAlgorithmKind,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub algorithm_for: enums::TransactionType,
}
