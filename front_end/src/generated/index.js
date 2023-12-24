// index.ts
function isValidJavaScript(jsCode) {
  try {
    new Function(jsCode);
    return true;
  } catch (err) {
    return false;
  }
}
function addStringStream(streamString) {
  let p = document.createElement("pre");
  let stream_list = document.getElementsByName("raw_html");
  if (stream_list.length === 0) {
    console.log("there is no raw html");
    return;
  }
  stream_list[stream_list.length - 1].appendChild(p);
  let stream = new ReadableStream({
    start(controller) {
      streamString.split(" ").forEach((word) => {
        controller.enqueue(new TextEncoder().encode(word + " "));
      });
      controller.close();
    }
  });
  let reader = stream.getReader();
  readStreamInChunks(reader, p);
}
async function readStreamInChunks(reader, p) {
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }
    let textChunk = new TextDecoder("utf-8").decode(value);
    p.textContent += textChunk;
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
}
export {
  isValidJavaScript,
  addStringStream
};
