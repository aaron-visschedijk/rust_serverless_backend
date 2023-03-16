use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use serde_dynamo;
use aws_sdk_dynamodb::{
    model::AttributeValue,
};
use aws_config;

use self::tables::Table;


pub mod models;
pub mod tables;

pub struct Client {
    client: aws_sdk_dynamodb::Client,
}


impl Client {
    pub fn new(aws_config: &aws_config::SdkConfig) -> Self {
        let dynamo_client = aws_sdk_dynamodb::Client::new(aws_config);
        Client {
            client: dynamo_client,
        }
    }

    pub async fn exists<'a, T: Serialize + Deserialize<'a>>(&self, table: &Table<'a, T>, key: &str, value: &str) -> bool {
        match self.get(table, key, value).await {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn put<'a, T: Serialize>(&self, table: &Table<'a, T>, item: &T) {
        let item_serialized: HashMap<String, AttributeValue> = serde_dynamo::to_item(&item).unwrap();
        self.client.put_item().table_name(table.name).set_item(Some(item_serialized)).send().await.unwrap();
    }

    pub async fn get<'a, T: Deserialize<'a>>(&self, table: &Table<'a, T>, key: &str, value: &str) -> Option<T> {
        if key == table.pk {
            return self.get_by_pk(table, key.to_string(), value).await;
        } else {
            return self.get_by_gsi(table, key.to_string(), value).await;
        }
    }

    async fn get_by_pk<'a, T: Deserialize<'a>>(&self, table: &Table<'a, T>, key: String, value: &str) -> Option<T>{
        let query_result = self.client.get_item()
            .table_name(&table.name.to_string())
            .key(
                key,
                AttributeValue::S(value.to_string())
            )
            .send()
            .await;
        
        let item = query_result.unwrap().item?;
        let serialized: T = serde_dynamo::from_item(item).unwrap();
        Some(serialized)
    }

    async fn get_by_gsi<'a, T: Deserialize<'a>>(&self, table: &Table<'a, T>, key: String, value: &str) -> Option<T> {
        let query_result = self.client.query()
            .table_name(table.name.to_string())
            .index_name(key.to_string())
            .key_condition_expression(
                format!("{} = :{}", key, key)
            )
            .expression_attribute_values(
                format!(":{}", key),
                AttributeValue::S(value.to_string())
            )
            .send()
            .await;
        
        for item in query_result.unwrap().items.unwrap() {
            let serialized: T = serde_dynamo::from_item(item).unwrap();
            return Some(serialized);
        }
        None                       
    }   
}