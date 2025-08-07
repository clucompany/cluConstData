<div id="header" align="center">

  <b>[cluconstdata]</b>
  
  (Compile-time macros for building persistent data structures in no_std and const environments. Supports buffer composition, and numeric formatting.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/cluConstData">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/cluConstData">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/cluConstData/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/cluConstData/actions/workflows/CI.yml) 


</div>
</div>

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
cluConstData = "2.1.2"
```

and this to your source code:

```rust
use cluConstData::const_data;
```
## Example

### multi_consts

Purpose: Combine any values at compile time.

```rust
use cluConstData::const_data;

const_data! {
	pub(crate) const URL: &str = "https://", "api.example.com";
	const TIMEOUT_MS: u32 = 3000;
	const HEADERS: &[&str] = &["Accept"], &["Content-Type"];
}

fn main() {
	println!("Endpoint: {URL}");
	println!("Timeout: {TIMEOUT_MS} ms");
	println!("Headers: {HEADERS:?}");
}
```

<a href="./examples">
  See all
</a>

## License

This project has a license according to (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2019-2025 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
