# Rust Web Example: _RESTful Axum_

##### Cole Nemec 2024.

> This is repository for my work in Bart Massey's CS410P taught in Spring '24 at PSU.

# Development

Reference `main.rs` to identify the API routes.
Right now, the application implements `GET`, `POST`, `DELETE`, and `PUT`. These can be interfaced with by making requests to the respective routes defined in main.rs.

When the app is running, go to the url to access a random Question.

## Run the code once:

Invoke `cargo run` and visit `http://localhost:3000` in a browser.

## Re-run on source change:

After running `cargo install cargo-watch`, invoke `cargo watch -q -c -w src/ -x run`.

# Sources:

I used Bart Massey's `knock-knock` project to design this draft of my question service. His code was referenced as I programmed my own material.

To better understand REST, I referenced the _What is a RESTful API_ page from the AWS website ([here](https://aws.amazon.com/what-is/restful-api/#:~:text=RESTful%20API%20is%20an%20interface,applications%20to%20perform%20various%20tasks.))

One of the riddles is from https://www.reddit.com/r/AskReddit/comments/1uddi7/what_are_some_of_the_hardest_riddles_you_know/.

One of the riddles is from https://www.rd.com/list/challenging-riddles/.

One of the riddles is from https://parade.com/947956/parade/riddles/.

I am not the original author of the image used as the "404" graphic. Unfortunately, I've lost the original source. But, it is not me.

# TODO:

- Get docker working.
  - Make networking work!
- Add web interface for all CRUD.
- Add comments.
- PostgresQL DB!
- Add error handling? // TODO is the current approach sufficient?
- Add tracing.
