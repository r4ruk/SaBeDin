use async_trait::async_trait;
use deadpool_lapin::{Manager, Pool};
use lapin::{Channel, ConnectionProperties,Queue};
use lapin::options::QueueDeclareOptions;
use crate::core::contracts::queue_types::{QueueRequestMessage, QueueResponseMessage};
use uuid::Uuid;
use crate::core::contracts::dependency_container::ExecutionContext;
use crate::core::contracts::errors::GeneralServerError;
use crate::queue_manager::{publisher, receiver};
use crate::queue_manager::publisher::PublishParams;

pub(crate) type Connection = deadpool::managed::Object<Manager>;

#[async_trait]
pub trait QueueManagerProvider: Send + Sync  {
    async fn get_from_queue(&self, context: &ExecutionContext, queue_name: &str) -> Result<QueueResponseMessage, GeneralServerError>;
    async fn publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<(), GeneralServerError>;
    async fn returning_publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<QueueResponseMessage, GeneralServerError>;
}



pub struct QueueManager {}



/// communication should be:
/// client
///     -> sends request to channel and provides return_channel (correlation_id)
///         -> external Service handles request
///             -> returns ResponseMessage to return_channel
/// this should ensure the queue to be exclusive for the single returning request.

impl QueueManager {
    /// initializes a new connectionpool
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

    /// creates queue on given parameters.
    pub async fn create_queue(&self, channel: Channel, name: &str, declaration_options: QueueDeclareOptions) -> Result<Queue, GeneralServerError> {
        println!("creating queue with name '{}'", name);
        return channel.queue_declare(name, declaration_options, Default::default()).await.map_err(|e| GeneralServerError{ message: format!("failed to create channel: {}", e) });
    }

    /// gets a connection from the pool of connections
    async fn get_queue_connection(&self, context: &ExecutionContext) -> Result<Connection, GeneralServerError> {
        let conn = context.queue.get().await;
        return if conn.is_ok() {
            Ok(conn.unwrap())
        } else {
            Err(GeneralServerError { message: "error getting connection from pool".to_string() })
        }
    }
}

#[async_trait]
impl QueueManagerProvider for QueueManager {
    async fn get_from_queue(&self, context: &ExecutionContext, queue_name: &str) -> Result<QueueResponseMessage, GeneralServerError> {
        let connection: Connection = self.get_queue_connection(&context).await?;
        let mut channel: Channel = connection.create_channel().await?;

        // receive a message on to defined queue
        channel = connection.create_channel().await?;

        // TODO not working yet, must be implemented, but too lazy atm.
        return receiver::receive_on_queue(self, channel, queue_name).await

    }

    /// basic publish function which handles general "POST" requests
    async fn publish(&self, context: &ExecutionContext, queue_name: &str, body: QueueRequestMessage) -> Result<(), GeneralServerError> {
        let conn = self.get_queue_connection(&context).await.map_err(|e| {
            eprintln!("could not get rmq con: {:?}", e);
            e
        })?;

        let channel:  Channel = conn.create_channel().await.map_err(|e| {
            println!("error in opening channel");
            e
        })?;

        let params = PublishParams {
            context,
            manager: &self,
            queue_name,
            channel,
            body,
        };
        return publisher::basic_publish(params).await
    }

    /// function which handles returning publishing functions. (usually that's "GET" requests to external services
    async fn returning_publish(&self, context: &ExecutionContext, queue_name: &str, mut body: QueueRequestMessage) -> Result<QueueResponseMessage, GeneralServerError> {
        let connection: Connection = self.get_queue_connection(&context).await?;
        let mut channel: Channel = connection.create_channel().await?;

        let correlation_id = Uuid::new_v4();
        body.correlation_id = correlation_id;

        // publish the message
        let params = PublishParams {
            context,
            manager: &self,
            queue_name,
            channel,
            body,
        };
        publisher::basic_publish(params).await?;

        // receive an 'immediate' response from external service
        channel = connection.create_channel().await?;
        let res = receiver::establish_temporary_listener(self, channel, correlation_id).await?;

        return Ok(res)
    }
}
