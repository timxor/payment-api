# API endpoint Documentation

## create a user
```
POST http://localhost:8000/user
```
Example payload:
```
{
	"firstName": "Test-api-003",
	"lastName": "Ethers",
	"userName": "@cultofcrypto",
	"number": 18472490853,
	"email": "frank@aol.com"
}
```

## read a user
```
GET http://localhost:8000/users
```

## update a user
```
PUT http://localhost:8000/user/:id
```

## delete a user
```
..
```
