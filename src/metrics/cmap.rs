use dashmap::DashMap;
use std::sync::Arc;
use std::fmt;
use anyhow::Result;



#[derive(Debug, Clone)]
pub struct CmapMertric {
    data: Arc<DashMap<String,i64>>,// Arc<Mutex<HashMap<String, i64>>> => Arc<DashMap<String, i64>>
}

impl Default for CmapMertric {
    fn default() -> Self {
        Self::new()
    }
}

impl CmapMertric {
    pub fn new() -> Self{
        CmapMertric{
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key:impl Into<String>) -> Result<()>{
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }   
}

impl fmt::Display for CmapMertric{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter(){
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}