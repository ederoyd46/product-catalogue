// use crate::types::DTORetrieveValue;
// use aws_sdk_dynamodb::{
//     error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
// };

// use log::{self, debug};

// use aws_sdk_dynamodb::Client;

// pub async fn retrieve_database_item(
//     table_name: &str,
//     retrieve_value: &DTORetrieveValue,
//     client: &Client,
// ) -> Result<GetItemOutput, SdkError<GetItemError>> {
//     debug!("About to get from DynamoDB {:?}", &retrieve_value.key);
//     client
//         .get_item()
//         .table_name(table_name)
//         .key("PK", AttributeValue::S(retrieve_value.key.to_string()))
//         .send()
//         .await
// }

// fn build_serde_value(attribute: &AttributeValue) -> Value {
//     if attribute.is_s() {
//         let val = attribute.as_s().unwrap();
//         Value::String(val.to_string())
//     } else if attribute.is_n() {
//         let val = attribute.as_n().unwrap();
//         // TODO Hate this..
//         match val.parse::<i64>() {
//             Ok(v) => Value::Number(Number::from(v)),
//             Err(_) => Value::Number(Number::from_f64(val.parse::<f64>().unwrap()).unwrap()),
//         }
//     } else if attribute.is_bool() {
//         let val = attribute.as_bool().unwrap().to_owned();
//         Value::Bool(val)
//     } else if attribute.is_m() {
//         let val = attribute.as_m().unwrap();
//         Value::Object(
//             val.iter()
//                 .map(|(k, v)| (k.to_string(), build_serde_value(v)))
//                 .collect(),
//         )
//     } else if attribute.is_l() {
//         let val = attribute.as_l().unwrap();
//         Value::Array(val.iter().map(|v| build_serde_value(v)).collect())
//     } else {
//         Value::Null
//     }
// }
