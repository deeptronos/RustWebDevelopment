# Rust Web Example: _PostgreSQL via Docker_

##### Cole Nemec 2024.

> This is repository for my work in Bart Massey's CS410P taught in Spring '24 at PSU.

# Run the Code

Call `docker-compose up` on a freshly-installed pull of this repo to start a PostgreSQL Docker container.

There may be issues with volumes or something... use the following two commands to fix it:

```
$ docker-compose down
$ docker-compose down --volumes
```

This allowed me to run `docker-compose up` successfully again.
