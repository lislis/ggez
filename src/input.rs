pub use sdl2::controller::GameController;
use sdl2::GameControllerSubsystem;
use sdl2::Sdl;
use std::collections::HashMap;
use std::fmt;

use context::Context;
use error::GameResult;


// before we can use gamepads (or joysticks) we need to "open" them
// then we have to keep them around
pub struct GamepadContext {
    gamepads: HashMap<i32, GameController>,
    // we need to keep the context around too
    #[allow(dead_code)]
    controller_ctx: GameControllerSubsystem,
}

impl fmt::Debug for GamepadContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<GamepadContext: {:p}>", self)
    }
}


impl GamepadContext {
    pub fn new(sdl_context: &Sdl) -> GameResult<Self> {
        let controller_ctx = sdl_context.game_controller()?;
        let joy_count = controller_ctx.num_joysticks()?;
        let mut gamepads = HashMap::new();
        for i in 0..joy_count {
            if controller_ctx.is_game_controller(i) {
                let controller: GameController = controller_ctx.open(i)?;
                // gamepad events use this instance_id
                let id = controller.instance_id();
                gamepads.insert(id, controller);
            }
        }
        Ok(GamepadContext {
            gamepads: gamepads,
            controller_ctx: controller_ctx,
        })
    }
}

/// returns the `GameController` associated with an instance id.
/// The `instance_id` can be obtained from `GamepadEvents` in the `EventHandler`
pub fn get_gamepad(ctx: &Context, instance_id: i32) -> Option<&GameController> {
    ctx.gamepad_context.gamepads.get(&instance_id)
}
