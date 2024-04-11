# `questdb-confstr`

## Format

Parser for a configuration string format used by
[QuestDB clients](https://questdb.io/docs/reference/clients/overview/).

The format is as follows:

```plain
service::key1=value1;key2=value2;key3=value3;
```

A few rules:
* The last semicolon is optional.
* Service name and keys are case-sensitive.
* Keys are ASCII alphanumeric and can contain underscores.
* Values are case-sensitive unicode strings which can contain any characters,
  * Except control characters (`0x00..=0x1f` and `0x7f..=0x9f`).
  * If semicolons `;` appears in a value, these are escaped as double semicolon `;;`.

## Grammar

```json
conf_str ::= service "::" params | service
service ::= identifier
params ::= param (";" param)* ";"?
param ::= key "=" value
key ::= identifier
value ::= { value_char }

identifier ::= alpha_num_under { alpha_num_under }
alpha_num_under ::= "a".."z" | "A".."Z" | "0".."9" | "_"
value_char ::= non_semicolon_char | escaped_semicolon
escaped_semicolon ::= ";;"
non_semicolon_char ::= ? any unicode character except ';', 0x00..=0x1f and 0x7f..=0x9f ?
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
