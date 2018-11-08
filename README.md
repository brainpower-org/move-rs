# move-rs

## Log

### 2018-11-8

* Added Building route
* Decided for CRUD naming scheme in the controller
* HTTP method names for route names
* Implemented FromFormValue trait for GeoCoordinate struct
* serde_dynamodb can't handle tuples, so we used a struct for geo coordinates
* All model struct attributes are now public. Good idea?

### 2018-11-1 

* Question: Where to define behaviour on data?
* Should not be implemented in routes
* Should not be implemented in traits attached to data
* Should be implemented as distinct entity
* We went with a controller pattern, for now there is one for the entire application at [src/move_app.rs](./src/move_app.rs)
