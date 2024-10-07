
<a id="readme-top"></a>
<div align="center">
<h1 align="center">Rust Pseudokude</h1>

  <p align="center">
    A dynamic sudoku solver made in Rust
  </p>

  <p align="center">
<img src="https://nthorn.com/images/rust-pseudokude/solving_example.gif" width="500">
</p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about">About</a>
    </li>
    <li>
      <a href="#getting-started">Getting started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
      </ul>
      <ul>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT -->
## About
Psuedokude is a dynamic sudoku solver that can solve boards of any size up to a `u16`. Psuedokude solves using stack-based backtracking as well as candidate analysis.

Read an in-depth analysis of this project [here](https://www.nthorn.com/articles/pseudokude).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- INSTALLATION -->
## Getting started

### Prerequisites

1. Download Rust\
1.1 Windows: download the installer [here](https://rustup.rs/)\
1.2 Linux/macOS: execute `curl https://sh.rustup.rs -sSf | sh`
3. Add the `colored` crate
   ```sh
   cargo add colored
   ```

### Installation

1. Clone/download the repo
   ```sh
   git clone https://github.com/nTh0rn/rust-pseudokude.git
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE -->
## Usage
1. Modify `let init = vec![]` within [`src/main.rs`](https://github.com/nTh0rn/rust-pseudokude/blob/master/src/main.rs) to the sudoku board of your choice.
2. Run in terminal using `cargo run` or build to `.exe` using `cargo build --release`.

<b>NOTE</b>
Example sudoku boards exist within [`src/main_timed.rs`](https://github.com/nTh0rn/rust-pseudokude/blob/master/src/main_timed.rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Nikolas Thornton - [nthorn.com](https://nthorn.com) - contact@nthorn.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>

