const UI_SHADOW = 20;
const UI_PADDING = 25;

const UI_SANS_TEXT_HEIGHT = 20;

/// The maximum alpha for 2D dialogs, notifications
const UI_MAX_ALPHA = 0.95;

fn get_sans_font() => {
   format!("{}px sans-serif", utils::to_browser_h(UI_SANS_TEXT_HEIGHT));
};

fn enable_shadow(ctx) {
   ctx.shadow_color = "rgba(0, 0, 0, 0.2)";
   ctx.shadow_blur = to_browser_h(UI_SHADOW);
   ctx.shadow_offset_y = to_browser_h(UI_SHADOW);
}

fn remove_shadow(ctx) {
   ctx.shadowBlur = 0;
   ctx.shadowOffsetY = 0;
};

const Theme = {
   background: {
      r: 0.9,
      g: 0.9,
      b: 0.9
   },
   settings: {
      background: {
         r: 0.1,
         g: 0.1,
         b: 0.2
      }
   }
};
