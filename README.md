# `questdb-confstr`

## Format

Parser for a configuration string format used by QuestDB clients.

The format is as follows:

```plain
service::key1=value1;key2=value2;key3=value3;
```

A few rules:
* The last semicolon is mandatory.
* Service name and keys are case-insensitive.
* Values are case-sensitive.
* A semicolon can't appear as a key.
* If a semicolon `;` appears in a value, escaped it as a double semicolon `;;`.

## Grammar

```plain
conf_str ::= service "::" params | service
service ::= identifier
params ::= param (";" param)* ";"
param ::= key "=" value
key ::= identifier
value ::= { value_char }

identifier ::= alpha { alphanumeric }
alpha ::= "a".."z" | "A".."Z"
alphanumeric ::= "a".."z" | "A".."Z" | "0".."9"
value_char ::= non_semicolon_char | escaped_semicolon
escaped_semicolon ::= ";;"
non_semicolon_char ::= ? any character except ';' ?
```

## Usage

### Add dependency to `Cargo.toml`

```shell
cargo add questdb-confstr
```

### Usage

Use the `parse_conf_str` function to parse into a `ConfStr` struct.

You can then access the service name as `&str` and parameters as a `&HashMap<String, String>`.

### Where we use it

We use this config parsing format in our [Rust, C, C++](https://github.com/questdb/c-questdb-client) and
[Python](https://github.com/questdb/py-questdb-client) clients.

We also use it to configure object stores for
[database replication](https://questdb.io/docs/operations/replication/#core-replication-settings).
