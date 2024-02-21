import React, { useState, useEffect } from 'react';
import {ApiClient} from "../../lib/client.ts";

const apiClient = new ApiClient();

const HealthCheck: React.FC = () => {
    const [data, setData] = useState<any>(null);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const responseData = await apiClient.get('/healthcheck');
                setData(responseData);
            } catch (error) {
                console.error('Error fetching data:', error);
            }
        };

        fetchData();
    }, []); // Empty dependency array means this effect runs only once, similar to componentDidMount

    return (
        <div>
            {data ? (
                <div>
                    {/* Render your fetched data here */}
                    <pre>{JSON.stringify(data, null, 2)}</pre>
                </div>
            ) : (
                <div>Loading...</div>
            )}
        </div>
    );
};

export default HealthCheck;
