use aws_sdk_dynamodb::model::AttributeValue;
use log::{self, debug};

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::PutItemError, output::PutItemOutput, types::SdkError};
use serde_json::value::to_value;
use serde_json::{Map, Value};

use crate::model::DataTransferObject;

pub async fn store_database_item(
    table_name: &str,
    data: &impl DataTransferObject,
    client: &Client,
) -> Result<PutItemOutput, SdkError<PutItemError>> {
    debug!("About to update DynamoDB");
    let metadata = to_value(data.get_metadata()).unwrap();
    let value = to_value(data).unwrap();

    client
        .put_item()
        .table_name(table_name)
        .item("PK", AttributeValue::S(data.get_key().to_string()))
        .item("metadata", build_attribute_value(&metadata))
        .item("value", build_attribute_value(&value))
        .send()
        .await
}

fn build_attribute_value(value: &Value) -> AttributeValue {
    match value {
        Value::String(val) => AttributeValue::S(val.to_string()),
        Value::Null => AttributeValue::Null(true),
        Value::Bool(val) => AttributeValue::Bool(val.clone()),
        Value::Number(val) => AttributeValue::N(val.to_string()),
        Value::Array(val) => build_dynamodb_array(val),
        Value::Object(val) => build_dynamodb_object(val),
    }
}

fn build_dynamodb_object(object: &Map<String, Value>) -> AttributeValue {
    AttributeValue::M(
        object
            .iter()
            .map(|(k, v)| (k.to_string(), build_attribute_value(v)))
            .collect(),
    )
}

fn build_dynamodb_array(object: &[Value]) -> AttributeValue {
    AttributeValue::L(object.iter().map(|v| build_attribute_value(v)).collect())
}

#[cfg(test)]
mod tests {
    use super::build_attribute_value;
    use aws_sdk_dynamodb::model::AttributeValue;
    use serde_json::{Number, Value};

    #[test]
    fn test_build_string_attribute_value() {
        let value = Value::String("String Value".to_string());
        let expected = AttributeValue::S("String Value".to_string());
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_number_attribute_value() {
        let value = Value::Number(Number::from(100));
        let expected = AttributeValue::N(100.to_string());
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_boolean_attribute_value() {
        let value = Value::Bool(true);
        let expected = AttributeValue::Bool(true);
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_object_attribute_value() {
        let value = Value::Object(
            vec![("boolean".to_string(), Value::Bool(true))]
                .into_iter()
                .collect(),
        );

        let expected = AttributeValue::M(
            vec![("boolean".to_string(), AttributeValue::Bool(true))]
                .into_iter()
                .collect(),
        );
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_list_attribute_value() {
        let list_value = vec![Value::Bool(true), Value::String("String Value".to_string())]
            .into_iter()
            .collect();

        let value = Value::Array(list_value);

        let expected = AttributeValue::L(
            vec![
                AttributeValue::Bool(true),
                AttributeValue::S("String Value".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(build_attribute_value(&value), expected);
    }
}
