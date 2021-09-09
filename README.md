# findrs

Search for patterns in files

## How to install

```
cargo install --locked --all-features \
  --git https://github.com/Aksh-Bansal-dev/findrs
```

## How to use

```
Usage: findrs <PATTERN> <PATH> [OPTIONS]

Options:
	-i, --ignore-case   	 Case insensitive search
	-h, --help          	 display help for findrs
	-d, --dir           	 search all the files in the directory
```

#### Example

```
cargo run -- -d -r "^then" ./demo
```

This will search for 'then' at the beginning of line in the demo folder recursively.
