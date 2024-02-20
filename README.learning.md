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

### Optimization
#### Build
Build first: 

```bash
cd frontend
ng build --configuration production --ssr
```
#### CSS
Using purgecss we can try **but beware, always test program works.** Assume we already built it (and we're in root directory of project)

```bash
npm i -g purgecss
cd frontend/dist/frontend
purgecss -css browser/*.css --content browser/index.html browser/*.js -o browser/
```

This will write back to `browser/`. If we don't want that, we can write in a new directory. **Make sure you create the folder manually, as it won't create for you.**

### 