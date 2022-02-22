use serde::{Deserialize, Serialize};

use crate::shared_structs::{AttributeDefinition, Projection, ProvisionedThroughput};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoSqlWorkBenchJson {
    #[serde(rename = "ModelName")]
    pub model_name: String,
    #[serde(rename = "ModelMetadata")]
    pub model_metadata: ModelMetadata,
    #[serde(rename = "DataModel")]
    pub data_model: Vec<DataModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataModel {
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "KeyAttributes")]
    pub key_attributes: KeyAttributes,
    #[serde(rename = "NonKeyAttributes")]
    pub non_key_attributes: Vec<AttributeDefinition>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Vec<GlobalSecondaryIndex>,
    #[serde(rename = "DataAccess")]
    pub data_access: DataAccess,
    #[serde(rename = "BillingMode")]
    pub billing_mode: String,
    #[serde(rename = "ProvisionedCapacitySettings")]
    pub provisioned_capacity_settings: ProvisionedCapacitySettings,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataAccess {
    #[serde(rename = "MySql")]
    pub my_sql: MySql,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MySql {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "KeyAttributes")]
    pub key_attributes: KeyAttributes,
    #[serde(rename = "Projection")]
    pub projection: Projection,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyAttributes {
    #[serde(rename = "PartitionKey")]
    pub partition_key: AttributeDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvisionedCapacitySettings {
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
    #[serde(rename = "AutoScalingRead")]
    pub auto_scaling_read: AutoScaling,
    #[serde(rename = "AutoScalingWrite")]
    pub auto_scaling_write: AutoScaling,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoScaling {
    #[serde(rename = "ScalableTargetRequest")]
    pub scalable_target_request: ScalableTargetRequest,
    #[serde(rename = "ScalingPolicyConfiguration")]
    pub scaling_policy_configuration: ScalingPolicyConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalableTargetRequest {
    #[serde(rename = "MinCapacity")]
    pub min_capacity: i64,
    #[serde(rename = "MaxCapacity")]
    pub max_capacity: i64,
    #[serde(rename = "ServiceRole")]
    pub service_role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalingPolicyConfiguration {
    #[serde(rename = "TargetValue")]
    pub target_value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelMetadata {
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "DateCreated")]
    pub date_created: String,
    #[serde(rename = "DateLastModified")]
    pub date_last_modified: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "AWSService")]
    pub aws_service: String,
    #[serde(rename = "Version")]
    pub version: String,
}
