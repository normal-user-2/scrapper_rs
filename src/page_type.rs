use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PageType {
    General,
    QA,
    StudentAdvice,
}
