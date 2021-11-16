//! Retrieves all the documents in a collection.
//! 
//! Retrieves the entire collection, and needs to be assigned to some variable.
//! Returns a Vec of the type that the collection is assigned to. 
//! 
//! # Examples
//! 
//! ```
//!     use mungos::{Mungos}
//!     use serde::{Serialize, Deserialize}
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {  
//!         foo: String
//!         bar: String
//!     }
//!     
//!     let db = Mungos.new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//! 
//!     let items = collection.get_full_collection().await.unwrap();
//! 
//! ```
//! 


use futures::stream::TryStreamExt;
use mongodb::error::Result;
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_full_collection(&self) -> Result<Vec<T>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}
