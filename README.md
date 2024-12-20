# CRUD Project

This project is a simple implementation of CRUD (Create, Read, Update, Delete) operations.

## Installation

* touch DB.db
* cargo install sqlx-cli --features sqlite
* sqlx migrate run --database-url sqlite://DB.db 
* cargo sqlx prepare --database-url sqlite://DB.db
