# SeaORM Exploration

## Features

- [x] Migrations
  - [x] create migrations (via cli)
    - Migrations can be created with the cli running from within the migrations package `cargo run -- generate <name of the migration>`.
    - The actual content of the migration needs to be filled but in the generated code an example is provided (Nice!)

  - [x] run migrations (via cli)
    - Migrations can be run equaly from the migrations folder by running `cargo run -- up`.
    - This commands runs all the migrations for more fine grain control check the help of the command or de docs

  - [x] run migrations (programatically)

- [x] Entity generation (Nice!)
