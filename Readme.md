# arrow 2 csv

A small utility to convert an apache arrow file to csv. 

## windows build (from debian)

	rustup target add x86_64-pc-windows-gnu
	cargo build --release --target x86_64-pc-windows-gnu

## Amazon lambda build (from debian)

https://aws.amazon.com/fr/blogs/opensource/rust-runtime-for-aws-lambda/

	sudo apt install musl musl-tools
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target x86_64-unknown-linux-musl