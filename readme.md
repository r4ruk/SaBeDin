
to test the service so far this curl command can be used:
curl -X POST 127.0.0.1:7878/client -d '{"method":"getbyid", "object":"{\"name\": \"John Doe\",\"age\":43,\"phones\":[\"44 1234567\",\"44 2345678\"]}", "params":["1"]}' -H 'Content-Type: application/json' -vv
