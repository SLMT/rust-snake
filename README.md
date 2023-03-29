# Rust Snake

A snake game written in Rust.

<img src="screenshot.png" />

## How To Run ?

First, install the Rust development evnironment from [here](https://www.rust-lang.org/tools/install) (if you do not have one).

Second, run the following command in the project directory:

```
> cargo run
```

### Version 2

If you want to play version 2 of the game, where the snake does not die when it touches the wall but
just passes through on the other side. And dies only when it touches itself.

Run following command for version 2

```
> cargo run -- 2
```


Enjoy!

## Game Controls & Rules

- Use the arrow keys on the keyboard to move the green snake.
- Eat the orange food to make the snake stronger (or longer).
- When the snake hits the border or itself, it dies.

## TODOs

- To have a better game-over screen

## License

Copyright (c) 2016-2022 rust-snake's contributors.

rust-snake is made available under the terms of [the MIT License](LICENSE.md).
