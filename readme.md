
to test the flow so far this curl command can be used:

GET Request:
curl -X POST 127.0.0.1:7878/client -d '{"method":"method", "object":"{\"id\": \"0f083f37-0693-42b8-8a3e-6b1dfa0221ff\",\"name\":\"TestUser\",\"phones\":[\"44 1234567\",\"44 2345678\"]}", "params":["1"]}' -H 'Content-Type: application/json' -vv


POST Request:
curl -X POST 127.0.0.1:7878/client \
-H 'Content-Type: application/json' \
-d '{"method":"method", "object":"{\"id\":\"0f083f37-0693-42b8-8a3e-6b1dfa0221ff\",\"name\":\"John Doe\",\"password\":\"password123\",\"email\":\"john@example.com\",\"age\":30}","params":["1"]}' \
-vv