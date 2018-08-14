# Rusty Notes

This is an attempt to create something similar to Google Keep, written in Rust.
The goal will be for it to be self contained in a single binary and easy to deploy to any server, using an Sqlite database.

## Plan

### Design
The design will be as such, create a common interface for creating, editing, saving and searching for notes. 

This interface will be exposed in this order:

  1. as a simple HTML and HTTP's GET and POST using Sqlite as a backend
  2. as WEBASSEMBLY using yew and perhaps something like stdweb to gain access to the IndexedDB store in the browser and to do service worker stuff
  3. JSON API with HTTP GET and POST

### Version 0.1

Simple HTTP GET and POST interface with Sqlite backend using rusqlite and actix-web.

### Version 0.2

Improve the user interface using some of the nice javascript libraries for laying out cards like google keep.

### Version 0.3

Implement WEBASSEMBLY IndexedDB and ServiceWorker with yew.
Might also need to get https://github.com/rustwasm/wasm-bindgen/issues/441 IndexedDB added to web-sys crate.

There's a page here which recommends an approach: https://developers.google.com/web/fundamentals/instant-and-offline/web-storage/offline-for-pwa it recommends using the Service Worker Cache API for JS/CSS/HTML
and using the IndexdDB for the page data.

Consider using one of the following javascript wrappers for IndexedDB or copying their design for a Rust crate based on web-sys:

  + https://github.com/jakearchibald/idb-keyval
  + https://github.com/localForage/localForage
  + https://github.com/jakearchibald/idb
  + https://github.com/pouchdb/pouchdb 
  + https://github.com/dfahlander/Dexie.js
  + https://github.com/erikolson186/zangodb
  + https://github.com/ujjwalguptaofficial/JsStore

### Version 0.4

JSON Export

### Version 0.5

HTML Fetching for links

### Version 0.6

JSON API