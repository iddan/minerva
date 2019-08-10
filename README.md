<center>
    <img src="logo.svg" height=300>
    <h1>Minerva</h1>
    <p>The Knowledge Store</p>
</center>

### Features

- Object data: no more tables, documents, keys
- Load ready-made knowledge sources

### Usage

#### Docker

```bash
docker run -p 31013:31013 iddan/minerva
```

#### Build from source

```
git clone git@github.com:iddan/minerva.git;
cd minerva;
cargo run;
```

### HTTP API

```http
GET /
```

Will return matching quads in the NQuads format

#### Parameters

**subject:** NQuads formatted quad subject to match by

**predicate:** NQuads formatted quad predicate to match by

**object:** NQuads formatted quad object to match by

**context:** NQuads formatted quad context to match by

```http
POST /
```

Will add quads in body, body should be in the NQuads format
