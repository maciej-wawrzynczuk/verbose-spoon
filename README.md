# My rag app

A simple RAG app in rust. For educational purpose.

## Premises

### Data flow

- Data source: local text, MD files.

### General

- Written in rust.
- async
- config from env

## Libraries

### Data loading

- walkdir

### Chunking

- text_splitter

### General libraries

- tokio
- color_eyre for top level error handling
- thiserror for reusable code
- figment
- serde
