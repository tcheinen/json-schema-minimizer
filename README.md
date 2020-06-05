## json-schema-minimizer

command line tool to reduce JSON into the minimum required to define the schema.  Values are reduced according to the following rules.

usage: `cat data.json | ./json-schema-minimizer > minimum.json`

| Value Type | Result |
| --- | --- |
| Null | Null |
| Bool | False |
| Number | 0 |
| String | "" |
| Array | Recursive reduction and duplicates are removed |
| Object | Recursive reduction and duplicates are removed |

for example
```json
{
  "favorite_color": null,
  "is_owner": true,
  "age": 25,
  "name": "Bob",
  "generic_list": [
    "hello",
    "howdy",
    55,
    []
  ],
  "generic_object": {
    "k1": "v1",
    "k2": "v2",
    "k3": 3
  }
}
```
becomes
```json
{
  "age": 0.0,
  "favorite_color": null,
  "generic_list": [
    "",
    0.0,
    []
  ],
  "generic_object": {
    "k1": "",
    "k3": 0.0
  },
  "is_owner": false,
  "name": ""
}
```

