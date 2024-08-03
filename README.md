# wslp

`wslp` is a command-line tool to convert Windows paths to WSL paths and vice versa.

## Installation

```shell
git clone https://github.com/h2cone/wslp.git
cd wslp
cargo install --path .
```

## Usage

To convert Windows paths to WSL paths:

```shell
wslp 'x:\path\to\file'
```

Output:

```shell
/mnt/x/path/to/file
```

To convert WSL paths to Windows paths, use the --reverse or -r flag:

```shell
wslp -r /mnt/x/path/to/file
```

Output:

```shell
x:\path\to\file
```
