create or replace get_agent_graph(agentid UUID, apikey UUID) returns json as $$
declare
    user_id_var UUID;
    agent_data json;
    key_expired BOOLEAN;
begin
    -- Check if the API key is provided
    if apikey is null then
        RAISE EXCEPTION 'API Key is missing';
    end if;

    -- Check if API key exists in the database and retrieve user_id if it does
    select user_id into user_id_var
    from api_keys
    where api_key = apikey;

    -- if user_id_var is NULL, it means the API key does not exist or user ID is NULL
    if user_id_var is null then
        RAISE EXCEPTION 'API Key does not exist or user ID is NULL for key: %', apikey;
    end if;

    -- Check if the API key has expired
    select not (expires_at > now()) into key_expired
    from api_keys
    where api_key = apikey;

    -- Raise exception if API key has expired
    if key_expired THEN
        RAISE EXCEPTION 'API Key has expired: %', apikey;
    end if;

    -- Retrieve agent data and cast it to JSON if it is JSONB
    select graph::json into agent_data
    from agents
    where agent_id = agentid and creator_id = user_id_var;

    -- if agent data was not found for this user, raise an exception
    if agent_data is null then
        RAISE EXCEPTION 'Agent ID: % not found or does not belong to user ID: %', agentid, user_id_var;
    end if;

    -- return the agent graph data as JSON
    return agent_data;
end;
$$ language sql;