// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

turbo::cfg! {r#"
    name = "Let him cook!"
    version = "1.0.0"
    author = "Enver Honk'o'Clock Podgorcevic"
    description = "Defend chef who is trying to cook something good!"
    [settings]
    resolution = [256, 256]
"#}

turbo::init! {
    struct GameState {
        crosshair_position: Vec2,
        time: u32,
        mice: Vec<struct Mouse {
            position: Vec2,
            speed: f32
        }>,
        frame: u32
    } = {
        Self {
            crosshair_position: Vec2::new(128.0, 128.0),
            time: 30,
            mice: vec![],
            frame: 0
        }
    }
}

turbo::go! {
    let mut state = GameState::load();
    
    let crosshair_increment = 2.0;
    
    // Set background color
    
    if gamepad(0).left.pressed() {
        state.crosshair_position.nudge_horizontal(-crosshair_increment);
    } else if gamepad(0).right.pressed() {
        state.crosshair_position.nudge_horizontal(crosshair_increment);
    }
    if gamepad(0).up.pressed() {
        state.crosshair_position.nudge_vertical(-crosshair_increment);
    } else if gamepad(0).down.pressed() {
        state.crosshair_position.nudge_vertical(crosshair_increment);
    }
    
    // add random mice
    if state.frame % 60 == 0 {
        state.mice.push(Mouse {
            position: Vec2::new((rand() % 256) as f32, 256.0),
            speed: 1.0
        });
    }
    
    // Update mouse positions and drop dead ones
    state.mice.retain_mut(|mouse| {
        mouse.position.y -= mouse.speed;

        if gamepad(0).start.just_pressed() {
            let dx = state.crosshair_position.x - mouse.position.x;
            let dy = state.crosshair_position.y - mouse.position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < 16.0 {
                return false
            } else {
                return true
            }
        }

        if mouse.position.y < 0.0 {
            false
        } else {
            true
        }
    });

    // draw floor
    for i in 0..8 {
        for j in 0..8 {
            sprite!("floor", i * 32, j * 32);
        }
    }
    
    // draw kitchen
    sprite!("kitchen", 0, 0);

    // Draw mice
    for mouse in &state.mice {
        sprite!("mouse", mouse.position.x as i32 - 16, mouse.position.y as i32 - 16);
    }

    // Draw simple crosshair
    sprite!("crosshair", state.crosshair_position.x as i32 - 16, state.crosshair_position.y as i32 - 16);
    
    // Save game state for the next frame
    state.frame += 1;
    state.save();
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    fn nudge_horizontal(&mut self, distance: f32) {
        self.x += distance;
        
        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > 255.0 {
            self.x = 255.0;
        }
    }
    
    fn nudge_vertical(&mut self, distance: f32) {
        self.y += distance;
        
        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y > 256.0 {
            self.y = 256.0;
        }
    }
}