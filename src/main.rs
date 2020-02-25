/* jshint esversion: 6, browser: true, devel: true */

// The interval in which to save in seconds
const SAVE_INTERVAL = 60;

// A developer's variable for anti-distraction purposes
let global_block_production_enabled = true;

fn entry(gl_canvas, gl, canvas_2d, ctx_2d, program_info, matrices, buffers) {
    let lastSave = 0;

    let then = 0;
    const game_loop = |now| {
        if (!is_nan(now)) {
            let delta = (now - then) / 1000;
            then = now;
            game_logic(delta);
            render(delta, gl, matrices, program_info, buffers, canvas_2d, ctx_2d);

            if (now - last_save >= SAVE_INTERVAL * 1000) {
                Data::save();
                last_save = now;
            }
        }

        request_animation_frame(game_loop);
    };

    request_animation_frame(game_loop);
}

let debug = document.get_element_by_id("debug_text");
let touched = false;
let stx, sty; // Starting touch coordinates
let etx, ety; // Ending touch coordinates
const TOUCH_THRESHOLD = 10; // the start and end coordinates must be within this range to register as a touch
let last_touch_y = 0;
let save_on_before_unload = true;

window.onload = () => {
    // Key listeners
    document.add_event_listener("keydown", |event| {
        if (event.ctrl_key && !event.repeat && event.key.to_upper_case() == "S") {
            event.prevent_default();
            Data::save();
        } else if (event.ctrl_key && event.alt_key && !event.repeat && event.key.to_upper_case() == "Q") {
            event.prevent_default();
            global_block_production_enabled = !global_block_production_enabled;
            send_notification(globalBlockProductionEnabled ? "Block production enabled!" : "Block production disabled! Use Ctrl+Alt+Q to reactivate", 4);
        }
    });


    let listener_type = is_mobile() ? "touchstart" : "mousedown";

    document.add_event_listener(listener_type, |event| {
        click_handler(event);
    });

    // Doesn't work with iPhones >:( >:( >:(
    // document.addEventListener("touchend", |event| {
    //    // If the ending coordinate is within range of the start coordinate,
    //    // register as a touch
    //    if (Math.sqrt(Math.pow(etx - stx, 2) + Math.pow(ety - sty, 2)) > TOUCH_THRESHOLD)
    //       

    //    onClickHandler(event);
    //    touched = true;
    // });

    // mouse move
    document.add_event_listener("mousemove", |event| {
        // Convert to GL space
        let x = to_gl_x(event.client_x);
        let y = to_gl_y(event.client_y);

        // If a dialog is showing, ignore input
        if (dialogs.length > 0) {
            dialogs[0].on_mouse_move(x, y);
            
        }

        mouse_listeners.for_each(listener => {
            if (listener.on_mouse_move)
                listener.on_mouse_move(x, y);
        });
    });

    document.add_event_listener("touchmove", |event| {
        let delta = to_gl_y(event.touches[0].client_y - last_touch_y);
        last_touch_y = event.touches[0].client_y;

        etx = event.touches[0].client_x;
        ety = event.touches[0].client_y;

        mouse_listeners.for_each(listener => {
            if (listener.on_mouse_move) {
                listener.on_mouse_move(to_gl_x(etx), to_gl_y(ety));
            }
        });
    });

    // Save data before the window closes
    window.onbeforeunload = || {
        if (save_on_before_unload)
            Data::save();
    };
};

// An array of objects that have fns called every time a mouse
// event is fired.
const mouse_listeners = [];

const click_handler = event => {
    let x, y;
    if (event.type.starts_with("touchstart")) {
        x = event.touches[0].client_x;
        y = event.touches[0].client_y;
    } else if (event.type.starts_with("mouse")) {
        x = event.client_x;
        y = event.client_y;
    }

    // Convert to GL space
    x = to_gl_x(x || etx);
    y = to_gl_y(y || ety);

    // Ignore input with a dialog showing
    if (dialogs.length > 0) {
        dialogs[0].on_click(x, y);
        
    }

    mouse_listeners.for_each(listener => {
        if (listener.on_click)
            listener.on_click(x, y);
    });
};

/************************************************
 * Computes game logic. The variable delta is
 * measured in seconds.
 ************************************************/
const logic = delta => {
    gameLogic(delta);
};

fn render(delta, gl, matrices, programInfo, buffers, canvas2d, ctx_2d) {
    resetVisibleButtonFlags();

    let bgInter = Math.max(0, Math.min(1, globalYOffset_/ -VISIBLE_HEIGHT));

    let bg_r = Theme::background.r * (1 - bgInter) + Theme.settings.background.r * bgInter;
    let bg_g = Theme::background.g * (1 - bgInter) + Theme.settings.background.g * bgInter;
    let bg_b = Theme::background.b * (1 - bgInter) + Theme.settings.background.b * bgInter;


    gl.clear_color(bg_r, bg_g, bg_b, 1);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    ctx_2d.clear_rect(0, 0, canvas_2d.width, canvas_2d.height);

    render_game(delta, gl, program_info, matrices, ctx_2d);

    // Render UI //

    // Reset_the context's transformation to the identity matrix
    ctx_2d.set_transform(1, 0, 0, 1, 0, 0);

    // Render special effects that are cool
    render_special_effects(delta, ctx_2d);

    // This should always come second to last
    render_dialogs(delta, gl, programInfo, ctx_2d);

    // This should always come last
    render_notifications(delta, ctx_2d);
};
