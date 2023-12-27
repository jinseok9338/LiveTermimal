export function isValidJavaScript(jsCode: string) {
  try {
    new Function(jsCode);
    return true;
  } catch (err) {
    return false;
  }
}



// document should exist in the global scope
export function addStringStream(streamString: string, el: HTMLElement) {
  let p = document.createElement("pre");
  el.appendChild(p);
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

async function readStreamInChunks(reader: ReadableStreamDefaultReader<any>, p: HTMLPreElement) {
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
