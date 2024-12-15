# cean

## how to run

- install [docker](https://docs.docker.com/desktop/setup/install/windows-install/)
- setup .env
- $docker compose up --build

## how to use

```http
POST http://{your_ip}:{your_port}/send

header:
{
    "Content-Type: application/json"
}

payload:
{
   "auth": "{your_auth}", //string
   "channel_id": {your_channel}, //int
   "content": "{your_message}" //string
}

```
