use async_trait::async_trait;
use chrono::{DateTime, Utc};
use async_std::stream::StreamExt;
use deadpool_lapin::{Manager, Pool};
use lapin::{ConnectionProperties, Consumer};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use uuid::Uuid;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;

type Connection = deadpool::managed::Object<Manager>;

#[async_trait]
pub trait QueueManagerProvider: Send + Sync  {
    async fn get_queue_connection(&self, context: ExecutionContext) -> Result<Connection, GeneralServerError>;
    async fn establish_temporary_listener(&self, context: ExecutionContext, queue_name: &str, correlation_id: Uuid)
        -> Result<ResponseMessage, GeneralServerError>;
}



pub struct QueueManager {}

// body should be the expected services deserializable object structure representation in JSON
struct RequestMessage {
    message_id: Uuid,
    headers: String,
    body: String,
    timestamp: DateTime<Utc>,
}

// body should be the expected services deserializable object structure representation in JSON
struct ResponseMessage {
    correlation_id: Uuid,
    body: String
}

// communication should be:
// client -> sends request to channel and provides return address channel
// -> Service handles request (probably only get requests...?!) -> returns ResponseMessage to return_channel
// correlation_id should be the same as message_id of the request, so it knows it's reading the right message.

impl QueueManager {
    pub async fn init() -> Pool {
        let addr = "amqp://raruk:test123@127.0.0.1:5672";
        // let conn = Connection::connect(addr, ConnectionProperties::default()).await.map_err(|_| println!("error happened") );
        let manager = Manager::new(addr, ConnectionProperties::default());
        let pool = deadpool::managed::Pool::builder(manager)
            .max_size(10)
            .build()
            .expect("can create pool");

        pool
    }
}

#[async_trait]
impl QueueManagerProvider for QueueManager {
      async fn get_queue_connection(&self, context: ExecutionContext) -> Result<Connection, GeneralServerError> {
        let conn = context.queue.get().await;
        if conn.is_ok() {
            return Ok(conn.unwrap())
        } else {
            return Err(GeneralServerError{ message: "error getting connection from pool".to_string()})
        }
    }

    async fn establish_temporary_listener(&self, context: ExecutionContext, queue_name: &str, correlation_id: Uuid)
        -> Result<ResponseMessage, GeneralServerError> {

        // TODO establish connection and await answer
        let rmq_con = self.get_queue_connection(context).await.map_err(|e| {
            eprintln!("could not get rmq con: {:?}", e);
            e
        })?;
        let channel = rmq_con.create_channel().await?;

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        println!("Declared queue {:?}", queue);

        let mut consumer: Consumer = channel
            .basic_consume(
                queue_name,
                &format!("{}",correlation_id),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut response_body= String::new();

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                println!("received msg: {:?}", delivery);
                response_body = String::from_utf8_lossy(&delivery.data).to_string();
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await?
            }
        }
        return Ok(ResponseMessage{ correlation_id, body: response_body.to_string() })
    }
}
