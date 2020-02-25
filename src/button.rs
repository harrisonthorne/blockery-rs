const BUTTON_DEFAULT_TEXT_HEIGHT = 25;
const BUTTON_DEPTH = Board.BLOCK_WIDTH;

let buttons = [];

fn resetVisibleButtonFlags() {
   buttons.forEach(|button| {
      button.visible = false;
   });
};

mouse_listeners.push({
   onClick: |mx, my| {
      buttons.forEach(|button| {
         if (button.visible)
            button.onClick(mx, my);
      });
   },
   onMouseMove: |mx, my| {
      buttons.forEach(|button| {
         if (button.visible)
            button.onMouseMove(mx, my);
      });
   }
});


struct Button {
   constructor(x, y, w, h, color, text, action) {
      self.x = x;
      self.y = y;
      self.w = w;
      self.h = h;
      self.y_offset_= 0;
      self.color = color;
      self.disabledColor = self.toGrayscale(color);
      self.text = text;
      self.action = action;
      self.typeface = "New Cicle Fina";
      self.fontSize = BUTTON_DEFAULT_TEXT_HEIGHT;
      self.enabled = true;

      self.hovering = false;
      self.pressInter = 0;
      self.pressInterVelocity = -1;
      self.liftInter = 0;

      // a flag to tell if the button has been rendered. it is
      // reset_at the beginning of every render loop to false,
      // and set_to true and the end of the button's
      // render() method
      self.visible = false;

      // add this button to the pile of buttons
      buttons.push(self);
   }

   onClick(mx, my) {
      if (!self.visible || !self.enabled)
         

      if (self.coordinateInBounds(mx, my)) {
         self.pressInterVelocity = 1;
         if (self.action) self.action();
      }
   }

   coordinateInBounds(x, y) {
       x >= self.x && x <= self.x + self.w && y >= self.y + self.y_offset_&& y <= self.y+self.y_offset + self.h
   }

   onMouseMove(mx, my) {
      if (!self.visible || !self.enabled) {
         self.hovering = false;
         
      }

      if (self.coordinateInBounds(mx, my)) {
         self.hovering = true;
      } else {
         self.hovering = false;
      }
   }

   toGrayscale(color) {
      let rgb = intToRGB(color);
      let gray = (rgb.r + rgb.g + rgb.b) / 3;

       rgbToInt(gray, gray, gray)
   }

   addHighlightToColor(color) {
      let addHighlight = self.liftInter * 0.05;
      let rgb = intToRGB(color);

      rgb.r += addHighlight;
      if (rgb.r > 1) rgb.r = 1;

      rgb.g += addHighlight;
      if (rgb.g > 1) rgb.g = 1;

      rgb.b += addHighlight;
      if (rgb.b > 1) rgb.b = 1;

       rgbToInt(rgb.r, rgb.g, rgb.b)
   }

   renderBody(gl, programInfo, z) {
      cube_mesh.set_color(self.enabled ? (self.addHighlightToColor(self.color)) : self.disabledColor, gl, programInfo);
      cube_mesh.render(gl, self.x, self.y + self.y_offset, z, self.w, self.h, BUTTON_DEPTH);
   }

   renderTopLayer(ctx_2d, buttonCenterX2D, buttonCenterY2D, toNewDepth) {
      if (!self.text) 
      ctx_2d.fillStyle = "white";
      ctx_2d.font = toBrowserH(toBrowserH(self.fontSize) * toNewDepth) + "px " + self.typeface;
      ctx_2d.textAlign = "center";
      ctx_2d.textBaseline = "middle";
      ctx_2d.fillText(self.text, buttonCenterX2D, buttonCenterY2D);
   }

   render(delta, gl, programInfo, ctx_2d, y_offset) {
      self.y_offset_= y_offset||0;

      // Interpolation properties for animation //

      self.pressInter += self.pressInterVelocity * delta * 10;
      if (self.pressInter < 0)
         self.pressInter = 0;
      else if (self.pressInter > 1) {
         self.pressInterVelocity *= -1;
         self.pressInter = 1;
      }

      self.liftInter += (self.hovering ? 1 : -1) * delta * 2;
      if (self.liftInter < 0)
         self.liftInter = 0;
      else if (self.liftInter > 1)
         self.liftInter = 1;

      let maxPress = BUTTON_DEPTH * 3 / 4;
      let maxLift = -10;

      // 3D button rendering //

      // get_z offset_of pressing the button
      let z = (self.pressInterVelocity > 0 ? cubicEaseOut(self.pressInter) : cubicEaseIn(self.pressInter)) * maxPress;
      // and displace further by the mouse-hover effect
      z += (self.hovering ? cubicEaseOut(self.liftInter) : cubicEaseIn(self.liftInter)) * maxLift;


      // Render the 3D button body
      self.renderBody(gl, programInfo, z);

      // 2D text rendering //

      // get_the proportionality constant (height on screen = k * (height of mesh / depth from viewer))
      // v can really be any number but 0
      let v = 300;
      let k = toBrowserH(v) * CAMERA_Z / v;
      let toNewDepth = (CAMERA_Z - z) / k;

      // Yes, the following lines of code convert to *gl space*
      // Calculate the new x coordinate in gl space
      let xDistanceFromCenter = to_browser_y(VISIBLE_WIDTH / 2 - (self.x + self.w / 2)) * toNewDepth;
      let buttonCenterX = VISIBLE_WIDTH / 2 - xDistanceFromCenter;

      // Calculate the new y coordinate in gl space
      let yDistanceFromCenter = to_browser_y(VISIBLE_HEIGHT / 2 - (self.y + self.y_offset_+ self.h / 2)) * toNewDepth;
      let buttonCenterY = VISIBLE_HEIGHT / 2 - yDistanceFromCenter;

      // Render the text!
      self.renderTopLayer(ctx_2d, to_browser_x(buttonCenterX), to_browser_y(buttonCenterY), toNewDepth);

      self.visible = true;
   }
}

struct ImageButton extends Button {
   constructor(x, y, w, h, color, img_src, action) {
      super(x, y, w, h, color, null, action);
      self.img = Image::new();
      if (img_src)
         self.img_src = img_src;
      self._enabled = true;
   }

   set_enabled(val) {
      self._enabled = val;
      if (val) {

      }
   }

   get_enabled() {
       self._enabled
   }

   set_img_src(val) {
      self.img.src = val;
   }

   get_img_src() {
       self.img.src
   }

   renderTopLayer(ctx_2d, buttonCenterX2D, buttonCenterY2D, toNewDepth) {
      if (!self.img_src) 
      let w = to_browser_w(to_browser_w(self.w - UI_PADDING) * toNewDepth);
      let h = toBrowserH(toBrowserH(self.h - UI_PADDING) * toNewDepth);
      ctx_2d.drawImage(self.img, buttonCenterX2D - w / 2, buttonCenterY2D - h / 2, w, h);
      super.renderTopLayer(ctx_2d, buttonCenterX2D, buttonCenterY2D, toNewDepth);
   }
}

struct ProgressButton extends Button {
   constructor(x, y, w, h, colorFill, colorEmpty, text, action) {
      super(x, y, w, h, null, text, action);
      self.colorFill = colorFill;
      self.colorEmpty = colorEmpty;
      /** An indicator from 0 to 1 */
      self.progress = 0.75;
   }

   renderBody(gl, programInfo, z, y_offset) {
      self.y_offset_= y_offset || 0;

      cube_mesh.set_color(self.addHighlightToColor(self.colorFill), gl, programInfo);
      cube_mesh.render(gl, self.x, self.y+self.y_offset, z, self.w * self.progress, self.h, BUTTON_DEPTH);

      cube_mesh.set_color(self.enabled ? self.addHighlightToColor(self.colorEmpty) : self.toGrayscale(self.colorEmpty), gl, programInfo);
      cube_mesh.render(gl, self.x + self.w * self.progress, self.y+self.y_offset, z, self.w * (1 - self.progress), self.h, BUTTON_DEPTH);
   }
}
