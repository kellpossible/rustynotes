# Rusty Notes

This is an attempt to create something similar to Google Keep, written in Rust.
The goal will be for it to be self contained in a single binary and easy to deploy to any server.

## Plan

### Design
The design will be as such, create a common interface for creating, editing, saving and searching for notes. 

This interface will be exposed in this order:

  1. as a simple HTML and HTTP's GET and POST using Sqlite as a backend
  2. as WEBASSEMBLY using yew and perhaps something like stdweb to gain access to the IndexedDB store in the browser and to do service worker stuff
  3. JSON API with HTTP GET and POST

### Version 0.1

 + [ ] Simple HTTP GET and POST interface

### Version 0.2

 + [ ] Markdown export (export all as a zip of markdown documents), and markdown sync. Use Markdown documents instead of the database for storing the Markdown, and have the ability to sync to changes to these documents. This means you can edit and view your notes on your file system, and synchronise them with backup systems like dropbox, google drive, git, etc.
 + [ ] Other data/index exported as plain text json
  
### Version 0.3

 + [ ] Improve the user interface using some of the nice javascript libraries for laying out cards like google keep
 + [ ] Password protection.

 + https://github.com/desandro/masonry

### Version ?

 + [ ] Implement WEBASSEMBLY IndexedDB and ServiceWorker with yew.

Might also need to get https://github.com/rustwasm/wasm-bindgen/issues/441 IndexedDB added to web-sys crate.

There's a page here which recommends an approach: https://developers.google.com/web/fundamentals/instant-and-offline/web-storage/offline-for-pwa it recommends using the Service Worker Cache API for JS/CSS/HTML
and using the IndexdDB for the page data.

Consider using one of the following JavaScript wrappers for IndexedDB or copying their design for a Rust crate based on web-sys:

  + https://github.com/jakearchibald/idb-keyval
  + https://github.com/localForage/localForage
  + https://github.com/jakearchibald/idb
  + https://github.com/pouchdb/pouchdb 
  + https://github.com/dfahlander/Dexie.js
  + https://github.com/erikolson186/zangodb
  + https://github.com/ujjwalguptaofficial/JsStore

Consider using https://github.com/markedjs/marked if I decide to go with a JavaScript solution here.

### Version ?

 + [ ] HTML Fetching for links

### Version ?

 + [ ] Time tracking

### Version ?

 + [ ] JSON Api
 + [ ] Syncing implementation to allow user to run rustynotes on their local machine and have it sync to an instance on a server
