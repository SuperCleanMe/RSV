# RSV
A Rust CSV parser. providing a custom API for serialization without needing other libraries

### Why should i use RSV?
A variety of reasons including the fact that this library does not require any external code to perform its task.

Some of the features are as follows:
- Serde free serialization<sup>[ref. 1](#reference-1-serde-free-serialization)</sup>
- easy to use API
- general, flexible output format 

### Installation
Add this line to your `Cargo.toml`
```toml
rsv = "0.0.5"
```
<br>

### Documentation
You can read the full documentation [here](https://docs.rs/rsv/)
#### Basic Usage
##### Parsing a basic CSV string:
```rust
use rsv::prelude::*;

// Create our input data
let input: String = "surname,initial,address,phone number\
Smith,A,\"14 Made up Drive, Made up City, Ohio\",216-235-3744\
Doe,J,\"15 Fake Street, Phonyville, Texas\",210-214-5737".to_string();

// Parse the `input` into `Content`
let content: Content = parse(input, ',', true);
```

##### Parsing a CSV file:
```rust
use rsv::prelude::*;

// Parse the `input` into `Content`
let content: Content = read("./path/to/file.csv", ',', true)?;
```

##### Pulling the data from `Content`:
the `Content` structure is incredibly flexible, you can use it as an `Iterator`, or as an `Index` such as an Array

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
now, i know what you are thinking "well that is not very rusty", dont worry for all of you die-hard rusty api users, i have an update coming soon for you, but for now im taking it slow and introducing this API first

### What's Next?
Well, i have already got some ideas of what i want to provide in the next minor release of this library, and so i will be working very heavily on that for now.

### How can i contribute?
For now, the only way i would like contributions is through github issues, as these are the easiest for me to track, however that does not rule out the possibilty of PRs, thus, you are welcome to help as much as you want, however i am much slower at responding to PRs than Issues

### Found a bug? Got a feature request?
well, if you found a bug, or you want to request a feature, the best place to report these bugs and suggest these features is with a [Github Issue](https://github.com/fatalcenturion/RSV/issues)

#### Congrat, you made it to the end, i just want to thank you for reading this information.

--------
### References
#### reference 1: "Serde free serialization"