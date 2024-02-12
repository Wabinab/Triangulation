# Learning

### Fontawesome icon big initial load
That's because we use SSR. We could fix it like this: 
Source: https://github.com/FortAwesome/react-fontawesome/issues/134

Inside **angular.json**: 
```json
"architect": {
  "build": {
    ...,
    "options": {
      ...,
      "styles": [
	"node_modules/@fortawesome/fontawesome-svg-core/styles.css", // -> this does the magic
	"src/styles.scss"
	],
    }
  }
}
```