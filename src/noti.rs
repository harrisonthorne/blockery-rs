/* jshint esversion: 6 */
/* global applyShadow, removeShadow, BOARD, getSansFont, UI_SHADOW, UI_MAX_ALPHA, quintEaseOut, getStatusBarX, getStatusBarWidth, getStatusBarHeight, to_browser_x, to_browser_y, to_browser_w, toBrowserH */

const NOTIFICATION_ANIMATE_DURATION = 0.5;
let notiMessage = "";
let notiDuration = 0;
let notiElapsedTime = 0;
const NOTIFICATION_HEIGHT = getStatusBarHeight() * 3 / 4;
fn renderNotifications(delta, ctx_2d) {
   if (notiElapsedTime > notiDuration) 
   notiElapsedTime += delta;

   let enter = quintEaseOut(notiElapsedTime / NOTIFICATION_ANIMATE_DURATION);
   enter = Math.min(1, enter);

   let exit = 1 - quintEaseOut((notiDuration - notiElapsedTime) / NOTIFICATION_ANIMATE_DURATION);
   exit = Math.max(0, exit);

   let x = to_browser_x(getStatusBarX());
   let y = to_browser_y(-(NOTIFICATION_HEIGHT + UI_SHADOW) * (1 - (enter - exit)));
   let w = to_browser_w(getStatusBarWidth());
   let h = toBrowserH(NOTIFICATION_HEIGHT);

   ctx_2d.save();

   applyShadow(ctx_2d);
   ctx_2d.fillStyle = "rgba(255, 255, 255, " + UI_MAX_ALPHA + ")";
   ctx_2d.fillRect(x, y, w, h);
   removeShadow(ctx_2d);

   ctx_2d.beginPath();
   ctx_2d.rect(x, y, w, h);
   ctx_2d.clip();

   // Render the message
   ctx_2d.fillStyle = "black";
   ctx_2d.font = getSansFont();
   ctx_2d.textAlign = "center";
   ctx_2d.textBaseline = "middle";
   let text_y = (h / 2) + (enter >= 1 ? y : y / 2);
   ctx_2d.fillText(notiMessage, x + w / 2, text_y);

   ctx_2d.restore();
};

fn sendNotification(message, duration) {
   notiMessage = message;
   if (duration) notiDuration = duration;
   else notiDuration = NOTIFICATION_ANIMATE_DURATION * 2;
   notiElapsedTime = 0;
};
