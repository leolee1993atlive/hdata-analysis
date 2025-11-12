use std::sync::Arc;

use rbatis::{RBatis, crud, impl_delete, impl_select, impl_update};
use rbdc::db::ExecResult;

use crate::biz::datasource::model::data_source::DataSource;

#[derive(Clone)]
pub struct DataSourceRepository {
    rb: Arc<RBatis>,
}

impl DataSourceRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    pub async fn select_all(&self) -> Result<Vec<DataSource>, rbatis::Error> {
        DataSource::select_all(&*self.rb).await
    }

    pub async fn select_by_id(
        &self,
        data_source_id: &i64,
    ) -> Result<Option<DataSource>, rbatis::Error> {
        DataSource::select_by_id(&*self.rb, data_source_id).await
    }

    pub async fn insert(
        &self,
        data_source: &DataSource,
    ) -> Result<ExecResult, rbatis::Error> {
        DataSource::insert(&*self.rb, data_source).await
    }

    pub async fn update_by_id(
        &self,
        data_source: &DataSource,
        data_source_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        DataSource::update_by_id(&*self.rb, data_source, data_source_id).await
    }

    pub async fn delete_by_id(
        &self,
        data_source_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        DataSource::delete_by_id(&*self.rb, data_source_id).await
    }
}

crud!(DataSource {});
impl_select!(
    DataSource{select_by_id(data_source_id: &i64) -> Option => "`where data_source_id = #{data_source_id} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_update!(
    DataSource{update_by_id(data_source_id: &i64) => "`where data_source_id = #{data_source_id} and deleted_by is null and deleted_date is null`"}
);
impl_delete!(
    DataSource{delete_by_id(data_source_id: &i64) => "`where data_source_id = #{data_source_id} and deleted_by is null and deleted_date is null`"}
);
