fn int_to_rgb(rgb) -> (u8, u8, u8) {
    let red = (rgb >> 16) / 255;
    let green = ((rgb >> 8) & 0xff) / 255;
    let blue = (rgb & 0xff) / 255;

    (red, green, blue)
};

fn int_to_rgba(rgba) -> (u8, u8, u8, f32) {
    let (r, g, b) = int_to_rgb(rgba);
    let a = (argb >> 24) / 255.0;

    (r, g, b, a)
};

fn int_to_rgba_text(rgba: i32) -> String {
    let (r, g, b, a) = int_to_rgba(rgba);

    format!("rgba({}, {}, {}, {})", r, g, b, a)
};

fn int_to_rgb_text(rgb: i32) -> String {
    let (r, g, b) = int_to_rgb(rgb);

    format!("rgba({}, {}, {}, 1)", r, g, b)
};

fn rgb_to_int(r: u8, g: u8, b: u8) -> i32 {
    (r << 16) + (g << 8) + (b)
}

fn window_width = () => window.inner_width;
fn window_height = () => window.inner_height;

/// Passes an error or, if no error, null and a 
/// result into callback.
fn load_text_resource(url, callback) -> Result<(), Box<dyn Error>> {
    let request = XMLHttpRequest::new();
    request.open("GET", url, true);
    request.onload = || {
        if (request.status < 200 || request.status > 299) {
            callback("Error: HTTP Status " + request.status + " on resource " + url);
        } else {
            callback(null, request.responseText);
        }
    };
    request.send();
}

/// Converts an x-coordinate in the browser window
/// to an x-coordinate usable in the WebGL matrix
/// space.
fn to_glx(client_x) -> f32 {
    (VISIBLE_HEIGHT * (2 * client_x - window_width())) / (2 * window_height()) + VISIBLE_WIDTH / 2
}

/// Converts an y-coordinate in the browser window
/// to an y-coordinate usable in the WebGL matrix
/// space.
fn to_gly(client_y) -> f32 { client_y * (VISIBLE_HEIGHT / window_height()) }

/// Converts an x-coordinate in the WebGL matrix
/// space to an x-coordinate usable in the browser
/// window.
fn to_browser_x(gl_x) -> f32 {
    (2 * window_height() * gl_x - window_height() * VISIBLE_WIDTH + window_width() * VISIBLE_HEIGHT) / (2 * VISIBLE_HEIGHT);
}

/// Converts an y-coordinate in the WebGL matrix
/// space to an y-coordinate usable in the browser
/// window.
fn to_browser_y(gl_y) -> f32 {
    gl_y * window_height() / VISIBLE_HEIGHT
}

/// Converts a width dimension in the WebGL matrix
/// space to a width dimension usable in the browser
/// window.
fn to_browser_w(gl_w) -> f32 {
    gl_w * window_height() / VISIBLE_HEIGHT
} 

/// Converts a height dimension in the WebGL matrix
/// space to a height dimension usable in the browser
/// window.
fn to_browser_h(gl_h) -> f32 {
    to_browser_y(gl_h)
}

fn is_mobile() {
    // TODO
    //  (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.user_agent))
};

fn get_wrapped_lines(ctx_2d, text, max_width) {
    if (!text) 

    // Convert to browser window space
    max_width = to_browser_w(max_width);
    let result = [];
    let words = text.split(" ");
    let line = "";

    for (let i = 0; i < words.length; i++) {
        let test = line + words[i];

        if (ctx_2d.measure_text(test).width <= max_width) {
            line = test + " ";
        } else {
            result.push(line);
            line = words[i] + " ";
        }
    }

    if (line.length > 0) {
        result.push(line);
    }

    result
};

fn cubic_ease_in(t: f32) -> f32 {
    t * t * t;
}

fn cubic_ease_out(t: f32) -> f32 {
    t--;

    t * t * t + 1
}

fn quint_ease_in(t) -> f32 {
    t * t * t * t * t
}


fn quint_ease_out(t) -> f32 {
    t--;

    t * t * t * t * t + 1
}
