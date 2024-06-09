import React, {useState} from "react";
import {ApiClient} from "../../lib/client.ts";

const LoginForm = () => {
    const [username, setUsername] = useState('');
    const [pwd, setPwd] = useState('');

    const handleLogin = async () => {
        console.log('logging in')
        const client = new ApiClient();
        await client.login(username, pwd).then((val) => {
            if(val) {
                document.getElementById('username').setAttribute('label', val);
            }
        });
    };

    return (
        <div>
            <form>
                <input
                    type="text"
                    placeholder="Username"
                    value={username}
                    onChange={e => setUsername(e.target.value)}
                />
                <input
                    type="password"
                    placeholder="Password"
                    value={pwd}
                    onChange={e => setPwd(e.target.value)}
                />
                <button type="button" onClick={handleLogin()}>Login</button>
            </form>
        </div>
    );
};