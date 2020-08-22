mod google;
mod mymemory;
mod translate;
mod yandex;

pub use google::*;
pub use mymemory::*;
pub use translate::*;
pub use yandex::*;

use crate::general;
use std::sync::Arc;

pub fn get_translator(
    source_language: &str,
    target_language: &str,
    state: &general::State,
) -> Arc<dyn Translate + Sync + Send> {
    if false {
        Arc::new(MyMemory::new(source_language, target_language))
    } else if false {
        Arc::new(Yandex::new(
            source_language,
            target_language,
            state.tranlation_pair.1.clone(),
        ))
    } else {
        Arc::new(Google::new(
            source_language,
            target_language,
            state.tranlation_pair.1.clone(),
        ))
    }
}
