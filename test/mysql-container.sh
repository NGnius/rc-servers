#!/bin/bash

# mysql://root:oj-pw@127.0.0.1:3306/openjam

docker run --detach --name openjam --env MYSQL_ROOT_PASSWORD=oj-pw --env MYSQL_DATABASE=openjam -p 3306:3306 mysql:latest
