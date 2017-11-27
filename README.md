# Raspberry Cook

Migrate <https://raspberry-cook.fr> from [Ruby on Rails](http://rubyonrails.org/) to [Rust](https://www.rust-lang.org/) & [Rocket](https://rocket.rs).

## Instalation

First you need to [install Rust](https://www.rust-lang.org/install.html). Then Clone the repository and go in the folder

~~~bash
$ git clone https://github.com/RaspberryCook/blazingfast_website
$ cd blazingfast_website
~~~

According to [Rocket], you need to use Nightly version of Rust

~~~bash
$ rustup override set nightly
~~~

Then install [diesel_cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) and run database migrations

~~~bash
$ cargo install diesel_cli --no-default-features --features sqlite
$ diesel setup
~~~

And now you can build project

~~~bash
$ cargo run
~~~


## License

[MIT](https://opensource.org/licenses/MIT)
