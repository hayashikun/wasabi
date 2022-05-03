import * as wasabi from "../../pkg";

const width = 32 * 4
const height = 32 * 3
const scale = 8

let app = wasabi.App.new(width, height)
console.log("wasabi.App.new")

let video = document.getElementById("video") as HTMLVideoElement;
navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
        width: {ideal: width * scale},
        height: {ideal: height * scale}
    }
}).then(function (stream) {
    video.srcObject = stream;
});

let hCanvas = document.getElementById("hidden-canvas") as HTMLCanvasElement;
hCanvas.width = width
hCanvas.height = height
let hCtx = hCanvas.getContext('2d')

let dCanvas = document.getElementById("display-canvas") as HTMLCanvasElement;
dCanvas.width = width * scale
dCanvas.height = height * scale
let dCtx = dCanvas.getContext('2d')

updateCanvas();

function updateCanvas() {
    hCtx!.drawImage(video, 0, 0, hCanvas.width, hCanvas.height);
    dCtx!.drawImage(video, 0, 0, dCanvas.width, dCanvas.height);
    let data = hCtx!.getImageData(0, 0, width, height).data
    let out = app.detect(Uint8Array.from(data))
    if (out.length > 0) {
        dCtx!.beginPath();
        dCtx!.rect(
            out[0] * scale, out[1] * scale,
            (out[2] - out[0]) * scale, (out[3] - out[1]) * scale
        );
        dCtx!.fillStyle = "#00FF00AA";
        dCtx!.fill();
        dCtx!.closePath();
    }
    requestAnimationFrame(updateCanvas)
}