mod cloud_formation;
mod nosqlworkbench;
mod shared_structs;
use std::collections::HashMap;
use std::io;

use crate::nosqlworkbench::NoSqlWorkBenchJson;

use crate::cloud_formation::{
    CloudFormation, GlobalSecondaryIndex, KeySchema, Properties, Resources,
};

fn main() {
    // 標準入力を受け付ける
    let mut input_json = String::new();
    io::stdin()
        .read_line(&mut input_json)
        .expect("標準入力の読み込みに失敗しました");
    let cloud_formation_json = create_cloud_formation_json(&input_json);
    println!("{}", cloud_formation_json);
}

fn create_cloud_formation_json(json: &str) -> String {
    let input_obj: NoSqlWorkBenchJson =
        serde_json::from_str(json).expect("jsonのパースに失敗しました");
    let data_model = &input_obj.data_model[0];
    let partition_key = &data_model.key_attributes.partition_key;

    let key_schema = vec![KeySchema {
        attribute_name: partition_key.attribute_name.clone(),
        key_type: String::from("HASH"),
    }];
    let partition_key = &data_model.key_attributes.partition_key;
    let attribute_definitions = {
        let mut definitions = vec![partition_key.clone()];
        for key in data_model.global_secondary_indexes.iter() {
            definitions.push(key.key_attributes.partition_key.clone());
        }
        definitions
    };
    let global_secondary_indexes = {
        data_model
            .global_secondary_indexes
            .iter()
            .map(|key| {
                let key_schema = vec![KeySchema {
                    attribute_name: key.key_attributes.partition_key.attribute_name.clone(),
                    key_type: String::from("HASH"),
                }];
                GlobalSecondaryIndex {
                    index_name: key.index_name.clone(),
                    key_schema,
                    projection: key.projection.clone(),
                    provisioned_throughput: data_model
                        .provisioned_capacity_settings
                        .provisioned_throughput
                        .clone(),
                }
            })
            .collect::<Vec<_>>()
    };

    let resources = {
        let mut map: HashMap<String, Resources> = HashMap::new();
        map.insert(
            data_model.table_name.clone(),
            Resources {
                resource_type: String::from("AWS::DynamoDB::Table"),
                properties: Properties {
                    key_schema,
                    attribute_definitions,
                    global_secondary_indexes,
                    billing_mode: data_model.billing_mode.clone(),
                    table_name: data_model.table_name.clone(),
                    provisioned_throughput: data_model
                        .provisioned_capacity_settings
                        .provisioned_throughput
                        .clone(),
                },
            },
        );
        map
    };

    let cloud_formation_obj: CloudFormation = CloudFormation {
        aws_template_format_version: String::from("2010-09-09"),
        resources,
    };
    serde_json::to_string(&cloud_formation_obj).expect("jsonの生成に失敗しました")
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;

    use crate::create_cloud_formation_json;

    #[test]
    fn test_create_json() {
        let json = "{\"ModelName\":\"ModelName\",\"ModelMetadata\":{\"Author\":\"name\",\"DateCreated\":\"Jun 02, 2021, 04:32 PM\",\"DateLastModified\":\"Feb 21, 2022, 05:42 PM\",\"Description\":\"Description\",\"AWSService\":\"Amazon DynamoDB\",\"Version\":\"3.0\"},\"DataModel\":[{\"TableName\":\"TableName\",\"KeyAttributes\":{\"PartitionKey\":{\"AttributeName\":\"referenceKey\",\"AttributeType\":\"S\"}},\"NonKeyAttributes\":[{\"AttributeName\":\"gsiKey\",\"AttributeType\":\"S\"},{\"AttributeName\":\"hoge\",\"AttributeType\":\"S\"},{\"AttributeName\":\"fuga\",\"AttributeType\":\"N\"}],\"GlobalSecondaryIndexes\":[{\"IndexName\":\"gsi\",\"KeyAttributes\":{\"PartitionKey\":{\"AttributeName\":\"gsiKey\",\"AttributeType\":\"S\"}},\"Projection\":{\"ProjectionType\":\"KEYS_ONLY\"}}],\"DataAccess\":{\"MySql\":{}},\"BillingMode\":\"PROVISIONED\",\"ProvisionedCapacitySettings\":{\"ProvisionedThroughput\":{\"ReadCapacityUnits\":5,\"WriteCapacityUnits\":5},\"AutoScalingRead\":{\"ScalableTargetRequest\":{\"MinCapacity\":1,\"MaxCapacity\":10,\"ServiceRole\":\"AWSServiceRoleForApplicationAutoScaling_DynamoDBTable\"},\"ScalingPolicyConfiguration\":{\"TargetValue\":70}},\"AutoScalingWrite\":{\"ScalableTargetRequest\":{\"MinCapacity\":1,\"MaxCapacity\":10,\"ServiceRole\":\"AWSServiceRoleForApplicationAutoScaling_DynamoDBTable\"},\"ScalingPolicyConfiguration\":{\"TargetValue\":70}}}}]}";
        let result = create_cloud_formation_json(json);
        let expect = "{\"AWSTemplateFormatVersion\":\"2010-09-09\",\"Resources\":{\"TableName\":{\"Type\":\"AWS::DynamoDB::Table\",\"Properties\":{\"KeySchema\":[{\"AttributeName\":\"referenceKey\",\"KeyType\":\"HASH\"}],\"AttributeDefinitions\":[{\"AttributeName\":\"referenceKey\",\"AttributeType\":\"S\"},{\"AttributeName\":\"gsiKey\",\"AttributeType\":\"S\"}],\"GlobalSecondaryIndexes\":[{\"IndexName\":\"gsi\",\"KeySchema\":[{\"AttributeName\":\"gsiKey\",\"KeyType\":\"HASH\"}],\"Projection\":{\"ProjectionType\":\"KEYS_ONLY\"},\"ProvisionedThroughput\":{\"ReadCapacityUnits\":5,\"WriteCapacityUnits\":5}}],\"BillingMode\":\"PROVISIONED\",\"TableName\":\"TableName\",\"ProvisionedThroughput\":{\"ReadCapacityUnits\":5,\"WriteCapacityUnits\":5}}}}}";
        assert_json_eq!(result, expect);
    }
}
