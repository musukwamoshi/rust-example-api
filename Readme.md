## Initates a new project
cargo new name-of-your-app

## Adds actix-web as a dependency
[dependencies]
actix-web = "4"

## Installs sea-orm-cli to run migrations
cargo install sea-orm-cli

## Initializes migrations directory
sea-orm-cli migrate init

## Creates a new migration
sea-orm-cli migrate generate name_of_migration

## Runs pending migrations
sea-orm-cli migrate up

## Generates new entities
sea-orm-cli generate entity -u protocol://username:password@localhost/bakery -o entity/src


Model is for read operations only. To perform insert, update, or delete, you need to use ActiveModel which attaches meta-data on each attribute.



