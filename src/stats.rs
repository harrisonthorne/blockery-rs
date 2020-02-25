struct Stats {

}

impl Stats {
    fn lifetime_blocks_collected(data: Data) -> u64{
        self.lifetime_blocks_produced_by_factories + data.lifetime_blocks_by_color.red + data.lifetime_blocks_by_color.orange + data.lifetime_blocks_by_color.green + data.lifetime_blocks_by_color.blue;
    }

    fn lifetime_blocks_produced_by_factories() {
        let total = 0;
        for (let prop in factories) {
            total += factories[prop].total_blocks_produced;
        }
         total
    }

    fn favorite_color(data: Data) {
        let max = Data.lifetime_blocks_by_color.red.max(Data.lifetime_blocks_by_color.orange.max(Data.lifetime_blocks_by_color.green).max(Data.lifetime_blocks_by_color.blue.max(Data.lifetime_blocks_by_color.toxic.max(Data.lifetime_blocks_by_color.golden)))));
        match max {
            data.lifetime_blocks_by_color.red => "Red",
            data.lifetime_blocks_by_color.orange => "Orange",
            data.lifetime_blocks_by_color.green => "Green",
            data.lifetime_blocks_by_color.blue => "Blue",
            data.lifetime_blocks_by_color.toxic => "Toxic blocks",
            data.lifetime_blocks_by_color.golden => "Golden blocks",
        }
    }
}

/// s the y-coordinate where the next statistic can be rendered
fn render_big_stat(stat_name: &str, stat_value: impl Display, ctx_2d, text_y, max_width) {
    ctx_2d.fill_style = "black";
    ctx_2d.text_baseline = "top";
    ctx_2d.text_align = "center";

    let big_font_size = UI_SANS_TEXT_HEIGHT * 1.25;
    let big_sans_serif = format!("{}px sans-serif", to_browser_h(big_font_size))
    ctx_2d.font = bigSansSerif;
    ctx_2d.fill_text(stat_name, utils::to_browser_x(VISIBLE_WIDTH / 2), to_browser_y(text_y));

    text_y += big_font_size;
    ctx_2d.font = format!("{}px Digital-7", utils::to_browser_h(75);
    ctx_2d.fill_text(statValue, to_browser_x(VISIBLE_WIDTH / 2), to_browser_y(text_y), max_width);

     text_y + 75 + UI_PADDING
}

/** s the y-coordinate where the next statistic can be rendered */
fn render_small_stat(stat_name, stat_value, ctx_2d, text_y) {
    ctx_2d.fill_style = "black";
    ctx_2d.text_baseline = "alphabetic";

    ctx_2d.text_align = "left";
    ctx_2d.fill_text(stat_name, to_browser_x(UI_PADDING), to_browser_y(text_y));

    ctx_2d.text_align = "right";
    ctx_2d.fill_text(stat_value, to_browser_x(VISIBLE_WIDTH - UI_PADDING), to_browser_y(text_y));

    ctx_2d.begin_path();
    ctx_2d.set_line_dash([1, 3]);
    ctx_2d.move_to(to_browser_x(UI_PADDING + 5) + ctx_2d.measure_text(statName).width, to_browser_y(text_y));
    ctx_2d.line_to(to_browser_x(VISIBLE_WIDTH - UI_PADDING - 5) - ctx_2d.measure_text(stat_value).width, to_browser_y(text_y));
    ctx_2d.stroke();
     text_y + UI_SANS_TEXT_HEIGHT * 1.25
};

fn render_stats(ctx_2d, y_offset) {
    let max_width = to_browser_w(VISIBLE_WIDTH - UI_PADDING * 2);
    let text_y = get_status_bar_height() + UI_PADDING + y_offset;

    text_y = render_big_stat("All blocks produced or collected", Math.floor(Stats.lifetime_blocks_collected).to_locale_string(), ctx_2d, text_y, max_width);
    // text_y = render_big_stat("Lifetime pollutants produced", Math.floor(Data.lifetime_pollution).to_locale_string(), ctx_2d, text_y, max_width);

    /* Render small stats */
    text_y += UI_PADDING;

    let total = 0;
    for (let prop in Data.lifetime_blocks_by_color)
        total += Data.lifetime_blocks_by_color[prop];

    ctx_2d.font = getSansFont();
    ctx_2d.strokeStyle = "black";
    ctx_2d.lineWidth = 1;

    // Block stats
    text_y = render_small_stat("All blocks collected by hand", total.to_locale_string(), ctx_2d, text_y);
    text_y = render_small_stat("Red blocks", Data.lifetime_blocks_by_color.red.to_locale_string(), ctx_2d, text_y);
    text_y = render_small_stat("Orange blocks", Data.lifetime_blocks_by_color.orange.to_locale_string(), ctx_2d, text_y);
    text_y = render_small_stat("Green blocks", Data.lifetime_blocks_by_color.green.to_locale_string(), ctx_2d, text_y);
    text_y = render_small_stat("Blue blocks", Data.lifetime_blocks_by_color.blue.to_locale_string(), ctx_2d, text_y);

    // Adds a gap
    text_y += UI_SANS_TEXT_HEIGHT * 1.25;

    // Click stats
    text_y = render_small_stat("Successful block clicks", Data.lifetime_clicks.successful.to_locale_string(), ctx_2d, text_y);
    text_y = render_small_stat("Failed block clicks", Data.lifetime_clicks.failed.to_locale_string(), ctx_2d, text_y);

    // Adds a gap
    text_y += UI_SANS_TEXT_HEIGHT * 1.25;

    // Factory stats
    let total_factories = 0;
    for (let prop in factories) {
        total_factories += factories[prop].amount_owned;
    }
    text_y = render_small_stat("Total factories owned", total_factories, ctx_2d, text_y);
    text_y = render_small_stat("All blocks produced by factories", (Stats.lifetime_blocks_produced_by_factories).floor().to_locale_string(), ctx_2d, text_y);

};
