export function isValidJavaScript(jsCode) {
  try {
    new Function(jsCode);
    return true;
  } catch (err) {
    return false;
  }
}

export function addStringStream(streamString) {
  let p = document.createElement("pre");
  // add class
  let stream_list = document.getElementsByName("raw_html");
  if (stream_list.length === 0) {
    console.log("there is no raw html");
    return;
  }
  stream_list[stream_list.length - 1].appendChild(p);
  // Create a new ReadableStream from the string
  let stream = new ReadableStream({
    start(controller) {
      // Split the string into words and enqueue each word separately
      streamString.split(" ").forEach((word) => {
        controller.enqueue(new TextEncoder().encode(word + " "));
      });
      controller.close();
    },
  });

  // Read the stream
  let reader = stream.getReader();
  readStreamInChunks(reader, p);
}

async function readStreamInChunks(reader, p) {
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }
    // Decode the value and append it to the paragraph
    let textChunk = new TextDecoder("utf-8").decode(value);
    p.textContent += textChunk;
    await new Promise((resolve) => setTimeout(resolve, 200)); // delay between chunks
  }
}
