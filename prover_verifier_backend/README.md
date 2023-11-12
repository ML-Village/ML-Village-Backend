# Prover Backend

## APIs

### `GET /models`

#### Returns

```json
[{
  "id": "string",
  "name": "string",
  "description": "string",
  "price": "string"
}]
```


### `POST /model/<model_id>/purchase"`

#### Path Params

`model_id` -> `string`

#### Body

```javascript
{
    api_key: "string",
}
```

#### Returns

```json
{
  "id": "string",
  "name": "string",
  "description": "string",
  "price": "string"
}
```

### `POST /upload_model`

#### Body

Multipart Form Data in the following format.

```javascript
{
    name: "string",
    description: "string",
    price: "string",
    onnx_file: File(),
}
```

#### Returns

```json
{
  "models_id": "string"
}
```

### `GET /proof/<proof_id>`

#### Path Params

`proof_id` -> `string`

#### Returns

A file containing the proof.

### `POST /create_user`

#### Body

None

#### Returns

```json
{
    "models": [
        "id": "string",
        "name": "string",
        "description": "string",
        "price": "string",
    ],
}
```

### `GET /me/<api_keys>`

#### Path Params

`api_key` -> `string`

#### Returns

```json
{
    "models": [
        "id": "string",
        "name": "string",
        "description": "string",
        "price": "string",
    ],
}
```
