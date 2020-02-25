/* jshint esversion: 6, devel: true, browser: true */

let resetButton;

fn initSettings() {
   let w = getStatusBarWidth() / 2;
   let h = getStatusBarHeight() / 2;
   resetButton = Button::new(VISIBLE_WIDTH / 2 - w / 2, VISIBLE_HEIGHT / 2 - h / 2, w, h, COLOR_RED, "Reset_everything", || {
      Data.reset();
   });
};

fn renderSettings(delta, gl, programInfo, ctx_2d, y_offset) {
   ctx_2d.fillStyle = "white";
   ctx_2d.textBaseline = "middle";
   ctx_2d.textAlign = "center";
   ctx_2d.font = toBrowserH(24) + "px New Cicle Fina";
   ctx_2d.fillText("(Do not push)", to_browser_x(VISIBLE_WIDTH / 2), to_browser_y(VISIBLE_HEIGHT / 2 + resetButton.h + y_offset));
   resetButton.render(delta, gl, programInfo, ctx_2d, y_offset);
};
