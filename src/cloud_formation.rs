use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared_structs::{AttributeDefinition, Projection, ProvisionedThroughput};

#[derive(Serialize, Deserialize, Debug)]
pub struct CloudFormation {
    #[serde(rename = "AWSTemplateFormatVersion")]
    pub aws_template_format_version: String,
    #[serde(rename = "Resources")]
    pub resources: HashMap<String, Resources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    #[serde(rename = "Type")]
    pub resource_type: String,
    #[serde(rename = "Properties")]
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: Vec<AttributeDefinition>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Vec<GlobalSecondaryIndex>,
    #[serde(rename = "BillingMode")]
    pub billing_mode: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    pub index_name: String,
    #[serde(rename = "KeySchema")]
    pub key_schema: Vec<KeySchema>,
    #[serde(rename = "Projection")]
    pub projection: Projection,
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ProvisionedThroughput,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeySchema {
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    #[serde(rename = "KeyType")]
    pub key_type: String,
}
