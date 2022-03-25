## Run

```shell
docker-compose up
```

To execute with cargo, need to set some env vars before:
```shell
APP_ENV=development \
APP_PORT=8000 \
APP_DB=app \
MONGODB_URL=mongodb://localhost:27017 \
USER_TOKEN_SALT=SECRET_USER_SALT \
cargo-run
```


## Register user
```shell
curl --request POST \
  --url http://localhost:8000/users/register \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "João",
	"password": "123",
	"email": "joaopintoneto@gmail.com"
}'
```

## Login
```shell
curl --request POST \
  --url http://localhost:8000/login \
  --header 'Content-Type: application/json' \
  --data '{
	"email": "joaopintoneto@gmail.com",
	"password": "123"
}'
```

## List users
```shell
curl --request GET \
  --url http://localhost:8000/users \
  --header 'Authorization: Bearer PASTE_THE_LOGIN_TOKEN_HERE'
```
