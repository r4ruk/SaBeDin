use sqlx::{Postgres, Transaction};
use crate::core::contracts::errors::GeneralServerError;

pub async fn handle_finish_transaction(result: Result<(), GeneralServerError>, transaction: Transaction<'_, Postgres>)
 -> Result<(), GeneralServerError>{
    return match result {
        Ok(_) => {
            if let Err(e) = transaction.commit().await {
                Err(GeneralServerError { message: e.to_string() })
            } else { Ok(()) }
        }
        Err(e) => {
            transaction.rollback().await?;
            Err(e)
        }
    }
}