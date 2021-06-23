let prev = document.getElementById("prev");
let next = document.getElementById("next");
let top_ = document.getElementById("top");

let start_x = 0;
let end_x = 0;

function process_touch() {
  if (end_x > start_x) {
    prev?.click();
  }
  if (end_x < start_x) {
    next?.click();
  }
}

window.addEventListener("keydown", function(event) {
  switch (event.key) {
    case "ArrowLeft": {
      prev?.click();
      break;
    }
    case "ArrowRight": {
      next?.click();
      break;
    }
    case "ArrowUp": {
      top_?.click();
      break;
    }
  }
}, true);

document.addEventListener("touchstart", function(event) {
  start_x = event.changedTouches[0].screenX;
});

document.addEventListener("touchend", function(event) {
  end_x = event.changedTouches[0].screenX;
  process_touch();
});
