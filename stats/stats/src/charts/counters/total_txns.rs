use crate::{
    charts::{
        create_chart,
        insert::DateValue,
        updater::{parse_and_sum, ChartDependentUpdater},
    },
    lines::NewTxns,
    Chart, UpdateError,
};
use async_trait::async_trait;
use entity::sea_orm_active_enums::ChartType;
use sea_orm::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct TotalTxns {
    parent: Arc<NewTxns>,
}

impl TotalTxns {
    pub fn new(parent: Arc<NewTxns>) -> Self {
        Self { parent }
    }
}

#[async_trait]
impl ChartDependentUpdater<NewTxns> for TotalTxns {
    fn parent(&self) -> Arc<NewTxns> {
        self.parent.clone()
    }

    async fn get_values(&self, parent_data: Vec<DateValue>) -> Result<Vec<DateValue>, UpdateError> {
        parse_and_sum::<i64>(parent_data, self.name(), self.parent.name())
    }
}

#[async_trait]
impl crate::Chart for TotalTxns {
    fn name(&self) -> &str {
        "totalTxns"
    }

    fn chart_type(&self) -> ChartType {
        ChartType::Counter
    }

    async fn create(&self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.parent.create(db).await?;
        create_chart(db, self.name().into(), self.chart_type()).await
    }

    async fn update(
        &self,
        db: &DatabaseConnection,
        blockscout: &DatabaseConnection,
        force_full: bool,
    ) -> Result<(), UpdateError> {
        self.update_with_values(db, blockscout, force_full).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::simple_test::simple_test_counter;

    #[tokio::test]
    #[ignore = "needs database to run"]
    async fn update_total_txns() {
        let counter = TotalTxns::new(Default::default());
        simple_test_counter("update_total_txns", counter, "22").await;
    }
}
