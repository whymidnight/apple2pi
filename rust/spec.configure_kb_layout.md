# configuring keyboard layout spec

## `layers.json`

```json
{
  "0x40": {
    "0xB0": {
      "remap": "0xB4"
    },
    "0xB1": {
      "macro": [
        {
          "0": {
            "action": "0xB2",
            "until": "2"
          },
          "1": {
            "action": "0xB3",
            "until": null
          },
          "2": {
            "action": "0xB4",
            "until": null
          }
        },
        "0xB3"
      ]
    }
  }
}
```
