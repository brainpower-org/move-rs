# move-rs

## Log

### 2018-11-1 

* Question: Where to define behaviour on data?
* Should not be implemented in routes
* Should not be implemented in traits attached to data
* Should be implemented as distinct entity
* We went with a controller pattern, for now there is one for the entire application at [src/move_app.rs](./src/move_app.rs)
