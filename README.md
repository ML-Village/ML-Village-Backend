# ML-Village-Backend

Backend Server for the ML Village Models.


## Setup

### Prover Backend

If its your first time running the backend, make sure to run the setup_db script before starting the server.

```bash
cargo run --bin setup_db
```

To run the prover backend, make sure you are at the workspace root (where this README.md is located) and run the following:

```bash
cargo run --bin server
```