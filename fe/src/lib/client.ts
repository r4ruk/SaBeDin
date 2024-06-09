import {LoginUserData} from "./definitions.ts";

const BASE_URL: string = "http://127.0.0.1:7878"

export class ApiClient {
    async get(client: string) {
       try {
           const response = await fetch(BASE_URL + client, {
               method: 'GET'});
           console.log(response);
           const client_response = await response.json();

           console.log(client_response);

           return client_response;
       } catch(error) {
           console.log(error);
       }
    }

    async login(email: string, pwd: string): Promise<string> {
        let login_data: LoginUserData = {
            email: email,
            password: pwd
        }
        try {
            const response = await fetch(BASE_URL + "/login", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(login_data)});
            console.log(response);
            const client_response = await response.json();

            console.log(client_response);

            return client_response;
        } catch(error) {
            console.log(error);
        }
    }
}
