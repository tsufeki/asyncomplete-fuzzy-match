asyncomplete-fuzzy-match
========================

Fuzzy matching for [asyncomplete.vim](https://github.com/prabirshrestha/asyncomplete.vim),
done asynchronously in a separate process (written in Rust).

Installation
------------

Clone the repo and run

```sh
cargo build --release
```

### With [vim-plug](https://github.com/junegunn/vim-plug)

```
Plug 'tsufeki/asyncomplete-fuzzy-match', {
    \ 'do': 'cargo build --release',
    \ }
```
