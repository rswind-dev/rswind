# Advanced Configuration

## `utilities`

The `utilities` field is an array of utility definitions. Each utility definition is an object with the following fields:
| Field | Type | Description |
| ----- | ---- | ----------- |
| `key` | `string` | The key of the utility, e.g. `bg` |
| `css` | `string` | The css handler for the utility, e.g. `background-color: $1` |
| `modifier` | `ValueDef` | The modifier for the utility, e.g. `bg-blue-500/50` |
| `theme` | `string` | The theme key for the utility, will read from `theme` by this key later, e.g. `colors` (camelCase) |
| `type` | `DataType \| PropertyKey` | Type of the utility, for inferring value of `arbitrary values`, could either be a css data type or a property key, e.g. `percentage`, `font-size` |
| `negative` | `boolean` | Whether the utility supports negative values |
| `fraction` | `boolean` | Whether the utility supports fraction values, e.g. `w-1/2` |

### Basic Usage
e.g. A `rswind.config.json` defining a custom utility like [outline-color](https://developer.mozilla.org/en-US/docs/Web/CSS/outline-color):

```json [rswind.config.json]
{
  "utilities": [
    {
      "key": "outline",
      "css": {
        "outline-color": "$0:color"
      },
      "theme": "colors"
    }
  ]
}
```

By simply defining a key with `outline` and theme of `colors`, utilities like `outline-blue-500` are able to use

### `type`

Adding `type` for this utility, we can use arbitrary value like `outline-[#333333]`

```json [rswind.config.json]
{
  "utilities": [
    {
      "key": "outline-color",
      "css": {
        "outline-color": "$0:color"
      },
      "theme": "colors",
      "type": "color"
    }
  ]
}
```

### `modifier`

Assume we need opacity, by defining `modifier`

`outline-blue-500/50` and `outline-red-600/[99]` are available

```json [rswind.config.json]
{
  "utilities": [
    {
      "key": "outline-color",
      "css": {
        "outline-color": "$0:color"
      },
      "theme": "colors",
      "type": "color",
      "modifier": {
        "theme": "opacity",
        "type": "percentage"
      }
    }
  ]
}
```
