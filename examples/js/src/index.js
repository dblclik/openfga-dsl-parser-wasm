function component() {
  const element = document.createElement('div');

  element.innerHTML = "hello dsl to json";
  wasm();
  return element;
}

async function wasm() {
  const moduleBytes = fetch("http://localhost:3000/openfga_dsl_parser.wasm");
  const module = await WebAssembly.compileStreaming(moduleBytes);
  const instance = await WebAssembly.instantiate(module, {});
  // obtain the module memory
  const linearMemory = instance.exports.memory;

  let encoder = new TextEncoder();
  let input = encoder.encode("type document");

  const buffer = new Uint8Array(linearMemory.buffer, 0, input.byteLength)
  buffer.set(input)
  // create a buffer starting at the reference to the exported string
  const offset = instance.exports.dsl_to_json(buffer.byteOffset, buffer.length);
  const stringBuffer = new Uint8Array(linearMemory.buffer, offset, instance.exports.doc_len());

  // create a string from this buffer
  let str = '';
  for (let i=0; i<stringBuffer.length; i++) {
    str += String.fromCharCode(stringBuffer[i]);
  }
  console.log(str)
}

document.body.appendChild(component());
