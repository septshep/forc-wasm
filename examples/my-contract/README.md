## How to install

```sh
npm install
```

## How to run in debug mode

Builds the project and opens it in a new browser tab.
Auto-reloads when the project changes.

```sh
npm start
```

## What does each file do?

* `package.json` contains the standard npm metadata. You put your JavaScript
  dependencies in here. You must change this file with your details (author,
  name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to
  change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook
  everything into Webpack, you don't need to change it).

* The `static` folder contains any files that you want copied as-is into the
  final build. It contains an `index.html` file which loads the `index.js` file.
