use std::str::FromStr;

use async_trait::async_trait;
use chrono::NaiveDate;
use sea_orm::{Database, DatabaseConnection, DbErr};
use stats_proto::blockscout::stats::v1::{
    stats_service_server::StatsService, Counters, GetCountersRequest, GetLineChartRequest,
    LineChart,
};
use tonic::{Request, Response, Status};

pub struct Service {
    db: DatabaseConnection,
}

impl Service {
    pub async fn new(db_url: &str) -> Result<Self, DbErr> {
        let db = Database::connect(db_url).await?;
        stats::mock::fill_mock_data(&db).await?;
        Ok(Self { db })
    }
}

#[async_trait]
impl StatsService for Service {
    async fn get_counters(
        &self,
        _request: Request<GetCountersRequest>,
    ) -> Result<Response<Counters>, Status> {
        let counters = stats::get_counters(&self.db)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        Ok(Response::new(counters))
    }

    async fn get_line_chart(
        &self,
        request: Request<GetLineChartRequest>,
    ) -> Result<Response<LineChart>, Status> {
        let request = request.into_inner();
        let from = request
            .from
            .and_then(|date| NaiveDate::from_str(&date).ok());
        let to = request.to.and_then(|date| NaiveDate::from_str(&date).ok());
        let chart = stats::get_chart_int(&self.db, &request.name, from, to)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        Ok(Response::new(chart))
    }
}
