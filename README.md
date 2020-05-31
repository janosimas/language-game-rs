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

# Acknowledgments

[Powered by Yandex.](http://translate.yandex.com/)

[Powered by Pixabay.](https://pixabay.com/)


## Icons

Flag Icons made by [Freepik](https://www.flaticon.com/authors/freepik) from [Flaticon](https://www.flaticon.com/)

Download image by [Papirus Development Team](https://github.com/PapirusDevelopmentTeam)
