# <div align="center">RSV</div>
<p align="center">
    <img src="https://github.com/fatalcenturion/RSV/workflows/Rust/badge.svg" alt="Build Status"/> <img alt="GitHub issues" src="https://img.shields.io/github/issues/fatalcenturion/RSV"/> <img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/fatalcenturion/RSV"/ >
</p>
<div align="center">A Rust CSV parser, built for the modern age.</div>


### Why should I use RSV?
A variety of reasons including the fact that this library does not require any external code to perform its task.

Some of the features are as follows:
- Serde<sup>[ref. 1](#reference-1-serde)</sup> free serialization<sup>[ref. 2](#reference-2-serde-free-serialization)</sup>
- easy to use API
- general, flexible output format 

### Installation
Add this line to your `Cargo.toml`:
```toml
rustsv = "0.1.4"
```

### Documentation
You can read the full documentation [here](https://docs.rs/rustsv/).
#### Basic Usage
##### Parsing a basic CSV string:
```rust
use rustsv::prelude::*;

// Create our input data
let input: String = "surname,initial,address,phone number\
Smith,A,\"14 Made up Drive, Made up City, Ohio\",216-235-3744\
Doe,J,\"15 Fake Street, Phonyville, Texas\",210-214-5737".to_string();

// Parse the `input` into `Content`
// The parameters are as follows:
// 1. Input: String   - The text you wish to parse
// 2. Delimiter: Char - The character to delimit by
// 3. Headers: Bool   - If the parser should use the first row in the file as headers
let content: Content = parse(input, ',', true);
```

##### Parsing a CSV file:
```rust
use rustsv::prelude::*;

// Parse the `input` into `Content`
// The parameters are as follows:
// 1. Path: String    - The path to the file ou wish to parse
// 2. Delimiter: Char - The character to delimit by
// 3. Headers: Bool   - If the parser should use the first row in the file as headers
let content: Content = read("./path/to/file.csv", ',', true)?;
```

##### Pulling the data from `Content`:
The `Content` structure is incredibly flexible, you can use it as an `Iterator`, or as an `Index` such as an Array.

##### As an `Iterator`:
```rust
let content: Content = read("./path/to/file.csv", ',', true);

for entry: Entry in content {
    //do stuff
}
```

##### As an `Index`:
```rust
let content: Content = read("./path/to/file.csv", ',', true);

let first_row: Entry = content[0];
```

##### What is an `Entry`?
The `Entry` structure is the container housing the individual pieces of data from each row in your input, it works similarly to it's `Content` parent.

##### Entry as an `Iterator`:

```rust
let content: Content = read("./path/to/file.csv", ',', true);

let first_row: Entry = content[0];

for key_pair: (String, String) in first_row {
    println!("Key: {}, Value: {}", key_pair.0, key_pair.1);
}
```

##### Entry as an `Index`:

```rust
let content: Content = read("./path/to/file.csv", ',', true);

let first_row: Entry = content[0];

let entry_name: String = first_row["name"];
```
Now, I know what you are thinking "well that is not very rusty", dont worry for all of you die-hard rusty api users, I have an update coming soon for you, but for now im taking it slow and introducing this API first.

### What's Next?
Well, I have already got some ideas of what I want to provide in the next minor release of this library, and so I will be working very heavily on that for now.

### How can I contribute?
For now, the only way I would like contributions is through GitHub issues, as these are the easiest for me to track, however that does not rule out the possibilty of PRs, thus, you are welcome to help as much as you want, however I am much slower at responding to PRs than Issues.

### Found a bug? Got a feature request?
Well, if you found a bug, or you want to request a feature, the best place to report these bugs and suggest these features is with a [Github Issue](https://github.com/fatalcenturion/RSV/issues).

#### Congratulations, you made it to the end, I just want to thank you for reading this information.

--------
### References

#### reference 1: "Serde"
Serde is a serialization and deserialization utility, providing the groundwork for almost all of the (de)serialization libraries out there, I am planning to integrate Serde compatibility into RSV in the next big update

#### reference 2: "Serde free serialization"
This simply means that you are not required to use Serde to serialize your data, a flaw that most of the other CSV libraries seem to share
