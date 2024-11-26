
# Advanced Configuration

## `utilities`

The `utilities` field is an array of utility definitions. Each utility definition is an object with the following fields:
| Field | Type | Description |
| ----- | ---- | ----------- |
| `key` | `string` | The key of the utility, e.g. `bg` |
| `css` | `string` | The css handler for the utility, e.g. `background-color: $1` |
| `modifier` | `ValueDef` | The modifier for the utility, e.g. `bg-blue-500/50` |
| `theme` | `string` | The theme key for the utility, will read from `theme` by this key later, e.g. `colors` (camelCase) |
| `validator` | `DataType \| PropertyKey` | The type validator for the utility, for inferring value of `arbitrary values`, could either be a css data type or a property key, e.g. `percentage`, `font-size` |
| `negative` | `boolean` | Whether the utility supports negative values |
| `fraction` | `boolean` | Whether the utility supports fraction values, e.g. `w-1/2` |

e.g. A `rswind.config.json` defining a custom utility like [outline-color](https://developer.mozilla.org/en-US/docs/Web/CSS/outline-color):

```json [rswind.config.json]
{
  "utilities": [
    {
      "key": "outline-color",
      "css": {
        "outline-color": "$0:color"
      },
      "modifier": {
        "type": "percentage",
        "theme": "opacity"
      },
      "theme": "colors",
      "type": "color"
    }
  ]
}
```
