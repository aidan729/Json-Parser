# JSON Parser in Rust (From Scratch)

This repository demonstrates a **minimal** JSON parser in Rust, implemented **from scratch** without relying on third-party JSON libraries. 
The code showcases how to build a lexer (tokenizer) and a recursive descent parser that can interpret JSON into a Rust data structure.

## Features

- **Lexer (Tokenizer)**: Converts the raw input string into a sequence of tokens (`{`, `}`, `,`, `:`, string literals, numbers, booleans, etc.).
- **Recursive-Descent Parser**: Consumes the token sequence and produces a `JsonValue` enum variant (`Null`, `Bool`, `Number`, `String`, `Array`, or `Object`).
- **Minimal Error Handling**: Provides basic error types for both lexing (`LexError`) and parsing (`ParseError`).

## Limitations & Future Improvements

- **String Escapes**: Only a few escape sequences (`\"`, `\\`, `\n`, `\t`, `\r`) are handled. A complete implementation would parse `\uXXXX` sequences for Unicode characters.
- **Numbers**: Parsed directly into `f64`. More robust handling could include distinguishing integers vs. floats or preventing certain malformed numeric formats.
- **Strict JSON Compliance**: This implementation is sufficient for many standard JSON structures but does not account for all possible edge cases (e.g., trailing commas, strict checking of leading zeros, etc.).
- **Error Reporting**: Errors do not currently include line/column info. This could be enhanced by tracking line and column positions in the tokenizer.

## How to Run

1. **Clone** the code into a new Rust project folder.

2. **Build and run**:
   ```bash
   cargo run
   ```
   
You should see output indicating the JSON was parsed successfully, along with a debug print of the resulting Rust data structure.

## Example

Here’s a sample JSON snippet that the parser can handle:

```json
{
  "name": "Alice",
  "age": 30,
  "married": false,
  "children": null,
  "pets": ["Cat", "Dog"],
  "address": { "city": "Wonderland", "zip": "12345" }
}
```

**Output** (pretty-printed debug output):

```plaintext
Successfully parsed JSON!
Object({
    "name": String("Alice"),
    "age": Number(30.0),
    "married": Bool(false),
    "children": Null,
    "pets": Array([
        String("Cat"),
        String("Dog")
    ]),
    "address": Object({
        "city": String("Wonderland"),
        "zip": String("12345")
    })
})
```

## Contributing

Feel free to open an issue or submit a pull request if you find bugs or want to add features (Some Ideas):


## License

This code is released under the [MIT License](LICENSE). You’re free to use and modify it in your own projects.
