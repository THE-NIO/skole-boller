use serde::Serialize;

pub struct Backend;

#[derive(Serialize)]
pub struct Remark {
    pub reason: String,
}

#[derive(Serialize)]
pub struct Class {
    pub name: String,
}

impl Backend {
    pub fn remarks() -> Vec<Remark> {
        vec!["programmering i norsk timen"]
            .into_iter()
            .map(ToString::to_string)
            .map(|reason| Remark { reason })
            .collect()
    }
    pub fn classes() -> Vec<Class> {
        vec!["math", "science"]
            .into_iter()
            .map(ToString::to_string)
            .map(|name| Class { name })
            .collect()
    }
}
