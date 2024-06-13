#!/bin/sh
for F in questions/*.json
do
    curl -d "@$F" -H "Content-Type: application/json" -X POST http://localhost:3000/api/question/add
done
