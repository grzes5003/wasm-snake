import { Universe } from "../pkg/wasm_snake_rust";

const pre = document.getElementById("snake-game-canvas");


// controls

const initText = "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium,\n" +
    "totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae\n" +
    "dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit,\n" +
    "sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam\n" +
    "est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius\n" +
    "modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima\n" +
    "veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea\n" +
    "commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil\n" +
    "molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?";

const universe = Universe.new_with_bcg(initText);
//universe.new_with_bcg(initText);

window.addEventListener('keydown', function(event) {
    switch (event.keyCode) {
        case 37: // Left
            universe.move_left();
            break;

        case 38: // Up
            universe.move_up();
            break;

        case 39: // Right
            universe.move_right();
            break;

        case 40: // Down
            universe.move_down();
            break;
    }
}, false);

let stop = false;
let frameCount = 0;
let fps, fpsInterval, startTime, now, then, elapsed;


// initialize the timer variables and start the animation

const startAnimating = (fps) => {
    fpsInterval = 1000 / fps;
    then = Date.now();
    startTime = then;
    requestAnimationFrame(renderLoop);
};

const renderLoop = () => {
    pre.textContent = universe.render();

    now = Date.now();
    elapsed = now - then;

    if (elapsed > fpsInterval) {
        // Get ready for next frame by setting then=now, but also adjust for your
        // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
        then = now - (elapsed % fpsInterval);

        // Put your drawing code here
        universe.tick();

    }
    requestAnimationFrame(renderLoop);
};

//requestAnimationFrame(renderLoop);
startAnimating(5);
