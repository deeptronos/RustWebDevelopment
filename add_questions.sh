#!/bin/sh
for F in questions/*.json
do
    curl -d "@$F" -H "Content-Type: application/json" -X POST http://0.0.0.0:3000/api/v1/question/add
done
