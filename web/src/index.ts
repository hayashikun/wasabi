import * as wasabi from "../../pkg";

const width = 32 * 4 * 2
const height = 32 * 3 * 2
const scale = 3

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
    let faces: [any] | undefined = JSON.parse(app.detect(Uint8Array.from(data))).faces;
    if (faces) {
        faces.forEach((f) => {
            dCtx!.beginPath();
            dCtx!.rect(
                f.x1 * scale, f.y1 * scale,
                (f.x2 - f.x1) * scale, (f.y2 - f.y1) * scale
            );
            dCtx!.fillStyle = "#00FF00AA";
            dCtx!.fill();
            dCtx!.closePath();
        })
    }

    requestAnimationFrame(updateCanvas)
}