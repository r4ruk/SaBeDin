# SaBeDin
## About The Project
This is a side project, which should serve as a general backend which handles
basic stuff a server should do.
#### The Idea
The idea is that this backend should cover the main functionality and handle MessageQueue communication with 'external'
Services which get the request params / POST body by MessageQueue. <br>
The main goal of this is to decouple the main app from "Plugin" functionality and make it as generic as possible so it can <br>
easily handle different other programming languages/project structures while still serving for the main critical tasks by itself.

Implemented with the help of \
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=red) 
![RabbitMQ](https://img.shields.io/badge/Rabbitmq-FF6600?style=for-the-badge&logo=rabbitmq&logoColor=white) 
![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=red) 
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black) 

### Installation
#### Docker
Run the docker containers, and make sure you provide the .env file in the main project directory using the command: \
`docker compose up`

#### Database Migrations
install the Migration using the sqlx-cli tool: \
if not installed yet install it via console with: `cargo install sqlx-cli` \
once installed the tables can be created with `sqlx migrate run`

#### Backend
run the `cargo build` and then `cargo run` command in your working directory.
Send the following curl commands and debug / see the outcome.\
It should be extendable and usable for different usecases. 
<br>


### Test
The easiest way to see how the core works you can run tests and debug the code.

#### Run UnitTests
I try to keep the tests updated, sorry if i fail at times, haha ;) \
just run `cargo test`.

#### RabbitMQ Management WebView
To see the rabbit mq management page you can open:
`localhost:15672` and log in with the credentials specified in the rabbitmq.conf file.

### Curl Calls flow

#### 1. Authentication:
Newly the user has to be registered first. i'll come back to that and provide a standarduser which can be used with \
the existing init sql configuration. i'll  add that as top priority in todo. \

##### 1.1 Register using curl command:
`curl -X POST "127.0.0.1:7878/register" --header "Content-Type: application/json" -d "{\"name\":\"user123\",\"email\":\"user@email.com\",\"password\":\"password\"}"`

##### 1.2 Login after registering
`curl -X POST "127.0.0.1:7878/login" --header "Content-Type: application/json" -d "{\"email\":\"user@email.com\",\"password\":\"password\"}"`

#### 2. GET Request:
`curl '127.0.0.1:7878/client?id=1' -H "Authorization: Bearer [AuthorizationTokenProvidedByAuthenticationCall]"` \
OR \
`curl "127.0.0.1:7878/healthcheck" --header "Authorization: Bearer [AuthorizationTokenProvidedByAuthenticationCall]"`

#### 3. POST Request:
`curl -X POST 127.0.0.1:7878/client \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer [AuthenticationTokenHere'] \
-d '{"method":"method", "object":"{\"id\":\"0f083f37-0693-42b8-8a3e-6b1dfa0221ff\",\"name\":\"John Doe\",\"password\":\"password123\",\"email\":\"john@example.com\",\"age\":30}","params":["1"]}' \
-vv`




### Basic Considerations
As usual there has to be a format defined for what to use for example on dates and here I decided to go for rfc3339 \
as this is a worldwide understandable format (YYYY-MM-DDTHH:MM:SS+HH:MM)

### Extending Tables and Persistence
To make the project build without a running db doing checks for syntactically and semantically valid queries for the current database \
and still ensuring safe & easy querying i built a dynamic raw-query creation logic which needs table names as which to work on. \
For this i created a static TABLE_NAMES variable which has to be extended as there is new Tables added.