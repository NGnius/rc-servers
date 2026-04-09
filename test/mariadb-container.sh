#!/bin/bash

# mysql://root:oj-pw@127.0.0.1:3306/openjam

docker run --detach --name openjam --env MARIADB_ROOT_PASSWORD=oj-pw --env MARIADB_DATABASE=openjam -p 3306:3306 mariadb:latest
