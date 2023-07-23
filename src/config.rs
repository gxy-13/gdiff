use std::collections::HashMap;
use reqwest::{Method, header::HeaderMap};
use url::Url;
use serde::{Deserialize, Serialize};
use tokio::fs;
use anyhow::Result;

// derive is a macro that implements some traits automatically for us
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {

    /*
        serde means "serialize/deserialize"
        flatten means "flatten the fields of the struct into the parent struct",
        {
        "name": "Alice",
        "age": 30,
        "address": {
            "street": "123 Main St",
            "city": "New York"
        }
        flatten: 
        {
        "name": "Alice",
        "age": 30,
        "street": "123 Main St",
        "city": "New York"
        }
} 
    
     */

    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    // Option<serde_json::Value> means that the value can be either None or Some(serde_json::Value)
    // Option is a enum type, it has two variants: None and Some(T). it makes the program more robust
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    // Vec<T> is a growable array type
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl DiffConfig {
    // async means that the function is asynchronous
    pub async fn load_yaml(path: &str) -> Result<Self> {
        // .await means that the function will wait for the result of the future
        // ? operator means that the function will return the error if the result is Err
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
}