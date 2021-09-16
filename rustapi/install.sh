#!/bin/bash

sudo apt-get install libsqlite3-dev
sudo apt install libpq-dev
sudo apt-get install libmysqlclient-dev

cargo install diesel_cli

diesel setup --database-url='db/sqlite.db'