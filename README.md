# My rag app

A simple RAG app in rust. For educational purpose.

## Premises

### Data flow

- Data source: local text, MD files.

### General

- Written in rust.
- async
- config from env

## Workflow

### Indexing

#### Ingesting

Input: a local directory with text files  
Output: collection of structs containing the file content and metadata.
How:

- Synchronously create list of files. Witch `walkdir`.
- Read with `tokio::fs`

#### Chunking

TBD

#### Embedding

TBD

#### Vector storage

TBD

## Libraries

### General libraries

- tokio
- color_eyre for top level error handling
- thiserror for reusable code
- figment
- serde
