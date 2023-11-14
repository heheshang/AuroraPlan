#!bin/bash
# Generate entity files of database `bakery` to `entity/src`
sea-orm-cli generate entity -u postgres://root:root@127.0.0.1:5432/dolphinscheduler --with-serde both --date-time-crate chrono --lib -o  ./src