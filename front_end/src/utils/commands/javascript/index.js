export function isValidJavaScript(jsCode) {
    try {
        new Function(jsCode);
        return true;
    } catch (err) {
        return false;
    }
}


export function addStringStream(streamString) {
    let string = "hello this is the world I am not a person I used to be"
    let p = document.createElement('p');
    let stream_list = document.getElementsByName('stream');
    if (stream_list.length === 0) {
        console.log("there is no raw html")
        return
    }
    console.log("there is raw html")
    stream_list[stream_list.length - 1].appendChild(p);
    // Create a new ReadableStream from the string
    let stream = new ReadableStream({
        start(controller) {
            controller.enqueue(new TextEncoder().encode(string));
            controller.close();
        }
    });

    // Read the stream
    let reader = stream.getReader();
    reader.read().then(({ done, value }) => {
        if (done) {
            return;
        }

        // Decode the value and append it to the paragraph
        let textChunk = new TextDecoder("utf-8").decode(value);
        p.textContent += textChunk;
    });
}
