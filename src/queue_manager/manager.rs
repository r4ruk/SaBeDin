use async_trait::async_trait;
use async_std::stream::StreamExt;
use deadpool_lapin::{Manager, Pool};
use lapin::{BasicProperties, Channel, ConnectionProperties, Consumer, Queue};
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use crate::core::contracts::queue_types::{QueueRequestMessage, QueueResponseMessage};
use uuid::Uuid;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;

type Connection = deadpool::managed::Object<Manager>;

#[async_trait]
pub trait QueueManagerProvider: Send + Sync  {
    async fn get_queue_connection(&self, context: &ExecutionContext) -> Result<Connection, GeneralServerError>;
    // async fn establish_temporary_listener(&self, context: ExecutionContext, queue_name: &str, correlation_id: Uuid)
    //     -> Result<ResponseMessage, GeneralServerError>;
    async fn basic_publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<(), GeneralServerError>;
    async fn returning_publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<QueueResponseMessage, GeneralServerError>;
}



pub struct QueueManager {}



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


    async fn establish_temporary_listener(&self, conn: Connection, correlation_id: Uuid)
                                          -> Result<QueueResponseMessage, GeneralServerError> {

        let channel = conn.create_channel().await?;

        let queue = channel
            .queue_declare(
                &correlation_id.to_string(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        println!("Declared queue {:?}", queue);

        let mut consumer: Consumer = channel
            .basic_consume(
                &correlation_id.to_string(),
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
        return Ok(QueueResponseMessage { correlation_id, body: response_body.to_string() })
    }

    async fn create_queue(&self, channel: Channel, name: &str, declaration_options: QueueDeclareOptions) -> Result<Queue, GeneralServerError> {
        return channel.queue_declare(name, declaration_options, Default::default()).await.map_err(|e| GeneralServerError{ message: "failed to create".to_string() });
    }
}

#[async_trait]
impl QueueManagerProvider for QueueManager {
      async fn get_queue_connection(&self, context: &ExecutionContext) -> Result<Connection, GeneralServerError> {
        let conn = context.queue.get().await;
          return if conn.is_ok() {
              Ok(conn.unwrap())
          } else {
              Err(GeneralServerError { message: "error getting connection from pool".to_string() })
          }
    }

    async fn basic_publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<(), GeneralServerError> {
        let conn = self.get_queue_connection(&context).await.map_err(|e| {
            eprintln!("could not get rmq con: {:?}", e);
            e
        })?;

        let channel:  Channel = conn.create_channel().await.map_err(|e| {
            println!("error in opening channel");
            e
        })?;


        let declare_options = QueueDeclareOptions {
            passive: true, // this param defines a creation if it does not exist and otherwise returns existing
            durable: true, // delete on shutdown?
            exclusive: false, // exclusive for the given connection, after closing delete
            auto_delete: false, // auto delete when there is no consumer connected.
            nowait: true, // does not get a return from the queue -> fire and forget
        };
        let _creation_result = self.create_queue(channel.clone(), queue_name,declare_options).await?;

        channel.basic_publish("", queue_name, BasicPublishOptions::default(), &serde_json::to_vec(&body).unwrap(), BasicProperties::default())
            .await
            .map_err(|e| {
                println!("cant publish: {}", e);
                e
            })?
            .await
            .map_err(|e|{
                println!("cant publish: {}", e);
                e
            })?;

        return Ok(())
    }

    async fn returning_publish(&self, context: &ExecutionContext, queue_name: &str, mut body: QueueRequestMessage) -> Result<QueueResponseMessage, GeneralServerError> {
        let correlation_id = Uuid::new_v4();

        let connection: Connection = self.get_queue_connection(&context).await?;

        body.correlation_id = correlation_id;

        self.basic_publish(&context, queue_name, body).await.map_err(|e| {
            e
        })?;
        let res = self.establish_temporary_listener(connection, correlation_id).await;
        return res
    }
}
