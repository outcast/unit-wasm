#!/usr/bin/env -S node
import { createServer } from 'http';
import { handle_request } from 'unit-wasm';

createServer( (req, res) => {
  try{
    handle_request(req, res)
    res.end();
  } catch(err) {
    res.writeHead(500, "WASM Error", {"Content-Type": "text/plain"});
    res.end("WASM Error\n\n"+err);
  }
}).listen(8000);