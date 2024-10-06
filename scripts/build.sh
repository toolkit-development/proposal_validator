cargo test candid -p proposal_validator

cargo build -p proposal_validator --release --target wasm32-unknown-unknown

gzip -c target/wasm32-unknown-unknown/release/proposal_validator.wasm > target/wasm32-unknown-unknown/release/proposal_validator.wasm.gz

cp target/wasm32-unknown-unknown/release/proposal_validator.wasm.gz wasm/proposal_validator.wasm.gz
