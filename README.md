# clinews

A simple command-line news reader written in Rust.

## Usage

First, you need to get an API key from [newsapi.org](https://newsapi.org). Then, create a `.env` file in the root of the project and add your API key to it like this:

```
API_KEY=your_actual_api_key
```

Then, you can run the application using `cargo run`:

```
_> cargo run
```

You can also specify the country and endpoint using the `--country` and `--endpoint` flags. For example, to get top headlines from Germany, you would run:

```
_> cargo run -- --country de
```

To search for articles about "rust" in the "everything" endpoint, you would run:

```
_> cargo run -- --endpoint everything --query rust
```

## Interactive Mode

After the articles are displayed, you can enter a number to open the corresponding article in your web browser.