# Requirements (V1)

## Entity
```json
{
  "id": "cdaec6b6-9240-45b3-a9a7-ac5ba8cfa213", 
  "label_code": "1231245123451", 
  "origin": "RJ", 
  "destiny": "SP"
}
```
* id: unique and automatic generated with an `uuid`;
* label_code: any value;
* origin: any value;
* destiny: any value;

## Endpoints

### Create Order
* Verb: POST
* Path values: none
* Body values: label_code, origin and destiny
* Response code: 201 (Created)
* Response body: not required

### Read Order
* Verb: GET
* Path values: id
* Body values: none
* Response code: 200 (OK)
* Response body: order

### Update Order
* Verb: PUT
* Path values: id
* Body values: label_code, origin and destiny
* Response code: 200 (OK)
* Response body: not required

### Delete Order
* Verb: DELETE
* Path values: id
* Body values: none
* Response code: 200 (OK)
* Response body: not required

## Data Persistence
Every data must be saved in an SQLite database;

## Project Structure
The project must isolate services, repositories and infra;

## Not Required (In This Version)
* Unit tests
* Integration tests
* Telemetry
* Logs
* Dockerized APIs
* Swagger