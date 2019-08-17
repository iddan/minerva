use crate::dataset::Dataset;
use crate::memory_dataset::MemoryDataset;
use crate::nquads_deserialize;
use std::sync::Mutex;

pub fn write<'a>(nquads: String, dataset_lock: &Mutex<MemoryDataset<'a>>) -> Result<(), String> {
    let quads = nquads_deserialize::deserialize(&nquads);
    let mut dataset = dataset_lock.lock().unwrap();
    for result in quads {
        match result {
            Err(error) => {
                return Err(error);
            }
            _ => dataset.insert(result.unwrap()),
        }
    }
    Ok(())
}
