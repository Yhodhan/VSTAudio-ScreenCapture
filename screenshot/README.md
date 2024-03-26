# Windows Screenshot

  The general idea of the crate is to directly use the windows api to get a bitmap of the screen or screens
  and then produce a png or bmp format screenshot, jpg format is not supported in this implementation mainly because it requires more data to be generated.

## Table of contents
- [Development](#development)
- [Dependencies](#dependencies)
- [Usage](#usage)

## Development
  The crate was developed with similar functionalities of crates with the same purpose.
  A possible optimization that can be done is to use SIMD to improve the performance of the read and write operations, but such a code is unsafe 
  and will turn the portability of the codebase more difficult, for instance ARM would be a challenge.
  
## Dependencies 

  - Git
  - Rust

## Usage 

  Inside the root directory simply run the command:
  ```
  cargo run 
  ```
  And the program will generate the image with the specified format inside the root directory
