#!/bin/bash

# postgres://postgres:oj-pw@127.0.0.1:5432/openjam

docker run --name openjam --env POSTGRES_PASSWORD=oj-pw --env POSTGRES_DB=openjam --detach -p 5432:5432 postgres:latest
