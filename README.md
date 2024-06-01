# Rust Web Example: _PostgreSQL via Docker_

##### Cole Nemec 2024.

> This is repository for my work in Bart Massey's CS410P taught in Spring '24 at PSU.

# Run the Code

**Required:** Ensure a file located at `db/password.txt` exists from the root directory. Write your choice of a database password in this file.

---

Call `docker-compose up --build` on a freshly-installed pull of this repo to start 2 Docker containers running the database and Rust web app.

There may be issues with volumes or something... use the following command to fix it:

```
$ docker-compose down --volumes
```

This allowed me to run `docker-compose up --build` successfully again.

---

# Usage:

**After Docker is running the servers...**

A templated webpage containing a random question can be visited by accessing the `<url>/` endpoint.

A question (in formatted JSON) can be added to the database by `POST`ing to the `<url>/api/question/add/` endpoint.

> See files in `questions/` for examples of the JSON format of a question.

> `add_questions.sh` is a script to add any `.json` file in `questions/` to the database via this endpoint.

View the DB's contents by accessing the `<url>/api/questions` endpoint.

# TODO

- Comments

  - `api.rs` ✅ TODO: further doc-ify
  - `questionbase.rs` ✅ TODO: further doc-ify
  - `main.rs`✅ TODO: further doc-ify
  - `question.rs` ✅
  - `web.rs` ✅

- Merge into main branch!
