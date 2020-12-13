import * as wasm from "rust-play";

//wasm.greet("Anders");
const Http = new XMLHttpRequest();
const url='http://localhost:5000';
Http.open("GET", url);
Http.send();

Http.onreadystatechange = (e) => {
  console.log(Http.responseText)
}
