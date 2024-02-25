# SaBeDin
## About The Project
This is a side project, which should serve as a general backend which handles
basic stuff a server should do.
#### The Idea
The idea is that this backend should cover the main functionality and handle MessageQueue communication with 'external'
Services which get the request params / POST body by MessageQueue. 

Implemented with the help of \
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=red) 
![RabbitMQ](https://img.shields.io/badge/Rabbitmq-FF6600?style=for-the-badge&logo=rabbitmq&logoColor=white) 
![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=red) 
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black) 

### Installation
run the `cargo build` and then `cargo run` command in your working directory.
Send the following curl commands and debug / see the outcome.\
It should be extendable and usable for different usecases.



### Test
The easiest way to see how the core works you can run tests and debug the code.

#### Run UnitTests
I try to keep the tests updated, sorry if i fail at times, haha ;) \
just run `cargo test`.

#### GET Request:
`curl '127.0.0.1:7878/client?id=1'`

#### POST Request:
`curl -X POST 127.0.0.1:7878/client \
-H 'Content-Type: application/json' \
-d '{"method":"method", "object":"{\"id\":\"0f083f37-0693-42b8-8a3e-6b1dfa0221ff\",\"name\":\"John Doe\",\"password\":\"password123\",\"email\":\"john@example.com\",\"age\":30}","params":["1"]}' \
-vv`

### Basic Considerations
As usual there has to be a format defined for what to use for example on dates and here I decided to go for rfc3339 \
as this is a worldwide understandable format (YYYY-MM-DDTHH:MM:SS+HH:MM)
