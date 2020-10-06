


var Scanner = {
    wasm: null,
    video: null,

    init: function (_wasm) {
        this.wasm = _wasm;
        window.wasm = _wasm;
        console.log("init scanner with wasm: ", this.wasm)
        console.log(this)

    },
    start() {
        console.log("start scanner with wasm")
        console.log(this)

        let video = window.document.getElementById("video");

        let constraints = {
            video: {
                facingMode: "environment",
            },
        };
        navigator.mediaDevices.getUserMedia(constraints).then(stream => {
            video.srcObject = stream;
        });

        const decodeQr = function (byteArray) {
            console.log("window.wasm", window.wasm);

            try {

                const output = window.wasm.decode_qr(byteArray);
                const usedOutput = output.includes("[Error]") ? null : output;
                console.log("output", usedOutput)
                if (usedOutput) {
                    let htmlElement = window.document.getElementById("video");
                    htmlElement.remove();
                    let result = window.document.getElementById("result");
                    let a = window.document.createElement('a');
                    let linkText = window.document.createTextNode(usedOutput);
                    a.appendChild(linkText);

                    a.title = usedOutput;
                    a.href = usedOutput;
                    result.appendChild(a);

                }


            } catch (err) {
                console.log("err", err)
            }
        }

        const captureImage = function () {
            let canvas = window.document.createElement("canvas");
            let scale = 0.25;
            canvas.width = video.videoWidth * scale;
            canvas.height = video.videoHeight * scale;
            canvas
                .getContext("2d")
                .drawImage(video, 0, 0, canvas.width, canvas.height);

            // let img = document.createElement("img");
            // img.src = canvas.toDataURL();
            window.myCanvas = canvas;
            // $output.appendChild(img);

            // const clampedByteArray = myCanvas.getContext("2d").getImageData(0, 0, 640, 480).data;

            canvas.toBlob(blob => {
                const reader = new FileReader();

                reader.addEventListener("loadend", () => {
                    const arrayBuffer = reader.result;
                    window.ab = arrayBuffer;

                    decodeQr(new Uint8Array(arrayBuffer));
                });
                reader.readAsArrayBuffer(blob);
            });
        }


        setInterval(captureImage, 300);
    }
}

export function init_scanner(_wasm) { Scanner.init(_wasm) }
export function start_scanner() { Scanner.start() } 
