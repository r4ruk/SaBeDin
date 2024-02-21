
const BASE_URL: string = "http://127.0.0.1:7878"

export class ApiClient {
    async get(client: string) {
       try {
           const response = await fetch(BASE_URL + client);
           const client_response = await response.json();

           console.log(client_response);

           return client_response;
       } catch(error) {
           console.log(error);
       }
    }
}
