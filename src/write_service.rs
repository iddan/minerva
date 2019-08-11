use std::sync::Mutex;
use crate::dataset::Dataset;
use crate::nquads_deserialize;

pub fn write(nquads: String, dataset_lock: &Mutex<Dataset>) -> Result<(), String> {
    let quads = nquads_deserialize::deserialize(&nquads);
    let mut dataset = dataset_lock.lock().unwrap();
    for result in quads {
        match result {
            Err(error) => {
                return Err(error);
            },
            _ => {
                dataset.insert(result.unwrap())
            }
        }
    }
    Ok(())
}