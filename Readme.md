# Tflite Scratchpad

A project that only exist to make it easy to peek under the hood. It was created for the sole purpose of adding tlfite support to tract [see related issue](https://github.com/sonos/tract/issues/1086)

## Getting started

I'm assuming you know how rust and cargo works so I'll skip that

### Python setup

if you are using [poetry](https://python-poetry.org/) it's as simple as

```sh
poetry install
poetry shell
```

If you want to install the dependencies manually or if you want to use the versions already installed on your system, they are:

- tflite
- tensorflow

### Getting the test data

the test_data is tracked via lfs so to pull that just run

```sh
git lfs install
```

### Recreating the tflite_generated.rs file

If you want to recreate the generated tflite rust code (if for example the schema [changes upstream](https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/schema/schema.fbs), or you want to play around with rust specific flags). follow these steps [ripped from this guide](https://orth.uk/flatbuffers/)

1. clone and compile the flatbuffer compiler
2. if you redownloaded the schema, change it's name from `schema.fbs` to `tflite.fbs`, otherwise skip
3. run `flatc --rust -o src/ tflite.fbs`, adding flags (such as `--rust-serialize`, `--gen-mutable`) as desired.
