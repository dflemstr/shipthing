use crate::config;
use crate::state;
use futures::lock;
use std::sync;

const SHIP_TEXTURE_WIDTH: f64 = 36.0;
const SHIP_TEXTURE_HEIGHT: f64 = 36.0;
const ENERGY_BAR_WIDTH: f64 = 40.0;
const ENERGY_BAR_HEIGHT: f64 = 6.0;

pub async fn run(state: sync::Arc<lock::Mutex<state::State>>) -> Result<(), failure::Error> {
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(
        concat!("shipthing ", env!("CARGO_PKG_VERSION")),
        [config::WORLD_WIDTH, config::WORLD_HEIGHT],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .map_err(|e| failure::err_msg(format!("{}", e)))?;

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets")?;
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf"))?;
    let main_text_style = piston_window::text::Text::new_color([0.0, 1.0, 0.0, 1.0], 24);
    let ship_text_style = piston_window::text::Text::new_color([1.0, 1.0, 1.0, 1.0], 10);

    let mut texture_context = piston_window::TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let ship_texture = piston_window::Texture::from_path(
        &mut texture_context,
        assets.join("ship.png"),
        piston_window::Flip::None,
        &piston_window::TextureSettings::new(),
    )
    .map_err(failure::err_msg)?;

    while let Some(e) = window.next() {
        log::trace!("event: {:?}", e);

        // Create a clone so we don't hold the lock while rendering
        let state = state.lock().await.clone();

        if let Some(result) =
            window.draw_2d::<_, _, Result<(), failure::Error>>(&e, |c, g, device| {
                use piston_window::character::CharacterCache;
                use piston_window::Transformed;

                piston_window::clear([0.0, 0.0, 0.0, 1.0], g);

                for ship in state.iter_ships() {
                    let (x, y) = ship.position;

                    let mut draw_ship = |x, y| -> Result<(), failure::Error> {
                        let width = glyphs.width(ship_text_style.font_size, &ship.name)?;
                        ship_text_style.draw(
                            &ship.name,
                            &mut glyphs,
                            &c.draw_state,
                            c.transform.trans(x - width / 2.0, y - SHIP_TEXTURE_HEIGHT),
                            g,
                        )?;
                        piston_window::rectangle(
                            [1.0, 1.0, 1.0, 1.0],
                            [
                                -ENERGY_BAR_WIDTH / 2.0,
                                -ENERGY_BAR_HEIGHT,
                                ENERGY_BAR_WIDTH * ship.energy / config::ENERGY_MAX_LEVEL,
                                ENERGY_BAR_HEIGHT,
                            ],
                            c.transform
                                .trans(x, y - SHIP_TEXTURE_HEIGHT / 2.0 - ENERGY_BAR_HEIGHT),
                            g,
                        );
                        piston_window::image(
                            &ship_texture,
                            c.transform
                                .trans(x, y)
                                .rot_rad(ship.direction)
                                .trans(-SHIP_TEXTURE_WIDTH / 2.0, -SHIP_TEXTURE_HEIGHT / 2.0),
                            g,
                        );
                        Ok(())
                    };

                    wrapping_draw(
                        x,
                        y,
                        SHIP_TEXTURE_WIDTH,
                        SHIP_TEXTURE_HEIGHT,
                        config::WORLD_WIDTH as f64,
                        config::WORLD_HEIGHT as f64,
                        |x, y| draw_ship(x, y),
                    )?;
                }

                main_text_style.draw(
                    "Hello world!",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(0.0, 24.0),
                    g,
                )?;

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);

                Ok(())
            })
        {
            result?;
        }
    }

    Ok(())
}

fn wrapping_draw(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    view_width: f64,
    view_height: f64,
    mut inner_draw: impl FnMut(f64, f64) -> Result<(), failure::Error>,
) -> Result<(), failure::Error> {
    let wraps_left = x < width;
    let wraps_right = x > view_width - width;
    let wraps_top = y < height;
    let wraps_bottom = y > view_height - height;

    inner_draw(x, y)?;
    if wraps_top {
        inner_draw(x, y + view_height)?;
    }
    if wraps_top && wraps_right {
        inner_draw(x - view_width, y + view_height)?;
    }
    if wraps_right {
        inner_draw(x - view_width, y)?;
    }
    if wraps_right && wraps_bottom {
        inner_draw(x - view_width, y - view_height)?;
    }
    if wraps_bottom {
        inner_draw(x, y - view_height)?;
    }
    if wraps_bottom && wraps_left {
        inner_draw(x + view_width, y - view_height)?;
    }
    if wraps_left {
        inner_draw(x + view_width, y)?;
    }
    if wraps_left && wraps_top {
        inner_draw(x + view_width, y + view_height)?;
    }
    Ok(())
}
