# language-game-rs

This is a simple game to help practice other languages.

The current state is the show a word in the source language and give options in a known language.

A few images are shown to help, the idea is to link visual and language memory.

## How to use it

The app uses online resources to translate and get the images, you'll need to create an account (free) and get an API key for

  - [Yandex](https://translate.yandex.com/developers)
  - [Pixabay](https://pixabay.com/api/docs/)

When you have the keys, create a file `.env` with the content:
```
YANDEX_KEY='your_key_to_yandex'
PIXABAY_KEY='your_key_to_pixabay'
```

This is still a toy project, so you'll need to compile yourself.

This link should give all the instructions: https://www.rust-lang.org/tools/install

After that go to the folder in your command line and execute `cargo run`.

## TODO (short term)

- I only added a file with words in German, should be easy to find lists of words in other languages.
- The translations are in english, the translations are online, any language should work.
- Improve error handling


# Acknowledgments

[Powered by Yandex.](http://translate.yandex.com/)

[Powered by Pixabay.](https://pixabay.com/)
