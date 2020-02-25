use rand::prelude::*;

/// The radius of a firework
const FIREWORK_SIZE = 100;

/// The size of each firework particle
const PARTICLE_SIZE = 10;

/// The duration of a firework in seconds
const FIREWORK_DURATION = 1.0;

/// The number of particles in a firework
const NUM_FIREWORK_PARTICLES = 16;

struct FireworkManager {
    fireworks: Vec<Firework>
}

struct Firework {
    pos: [f32; 2],
    interpolation: f32, // 0 - 1
    color: BlockColor,
}

impl Firework {
   pub fn new(x: f32, y: f32) {
      let color = BlockColor::random;
      firework_manager.push(self);
   }

   fn is_gone(&self) {
       self.interpolation >= 1
   }

   render(delta, ctx_2d) {
      // interpolate cuz we're cool
      if (self.interpolation < 1) {
         self.interpolation += delta / FIREWORK_DURATION;
         if (self.interpolation > 1)
            self.interpolation = 1;
      }

      // get_the quintic value of the animation interpolation
      let t = quintEaseOut(self.interpolation);
      // the calculated particle size (in browser space)
      let s = toBrowserH((1 - t * t * t) * PARTICLE_SIZE);
      // distance of a particle from the center of the firework
      let d = t * FIREWORK_SIZE;

      // draw all the particles yay
      ctx_2d.fillStyle = intToRGBText(self.color);
      for (let i = 0; i < NUM_FIREWORK_PARTICLES; i++) {
         let angle = 2 * Math.PI * i / NUM_FIREWORK_PARTICLES;
         let particleX = self.x + Math.sin(angle) * d;
         let particleY = self.y + Math.cos(angle) * d;
         ctx_2d.fillRect(to_browser_x(particleX - s / 2), to_browser_y(particleY - s / 2), s, s);
      }
   }
}

fn renderSpecialEffects(delta, ctx_2d) {
   // render fireworks
   fireworkManager.forEach(|firework| {
      firework.render(delta, ctx_2d);
      if (firework.isGone())
         fireworkManager.splice(fireworkManager.indexOf(firework), 1);
   });
}
