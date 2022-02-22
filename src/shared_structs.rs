use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Projection {
    #[serde(rename = "ProjectionType")]
    pub projection_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    N,
    S,
    #[serde(rename = "BOOL")]
    Bool,
    B,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeDefinition {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "AttributeType")]
    pub attribute_type: AttributeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProvisionedThroughput {
    #[serde(rename = "ReadCapacityUnits")]
    pub read_capacity_units: i64,
    #[serde(rename = "WriteCapacityUnits")]
    pub write_capacity_units: i64,
}
