use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, EnumIter, DeriveActiveEnum, PartialEq, Eq, Hash)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Site {
    #[sea_orm(string_value = "jfl")]
    JFL,
}

#[derive(Debug, Clone, EnumIter, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum PageType {
    #[sea_orm(string_value = "general")]
    General,
    #[sea_orm(string_value = "qa")]
    QA,
    #[sea_orm(string_value = "student-advice")]
    StudentAdvice,
}
