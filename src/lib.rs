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
        chef: struct Chef {
            position: Vec2,
            speed: f32,
            lives: u32
        },
        frame: u32,
        points: u32,
        game_over: bool
    } = {
        Self {
            crosshair_position: Vec2::new(128.0, 128.0),
            time: 30,
            mice: vec![],
            chef: Chef {
                position: Vec2::new(128.0, 32.0),
                speed: 1.0,
                lives: 3
            },
            frame: 0,
            points: 0,
            game_over: false,
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
    
    state.chef.move_chef();

    // Update mouse positions and drop dead ones
    state.mice.retain_mut(|mouse| {
        mouse.position.y -= mouse.speed;

        if gamepad(0).start.just_pressed() {
            let dx = state.crosshair_position.x - mouse.position.x;
            let dy = state.crosshair_position.y - mouse.position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < 16.0 {
                state.points += 1;
                return false
            } else {
                return true
            }
        }

        if mouse.position.y < 32.0 {
            // check collision with chef
            let dx = state.chef.position.x - mouse.position.x;
            let dy = state.chef.position.y - mouse.position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < 16.0 {
                state.chef.lives -= 1;
                if state.chef.lives == 0 {
                    state.game_over = true;
                }
            }
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
    
    if state.game_over {
    } else {
        // Draw chef
        sprite!("chef", state.chef.position.x as i32 - 16, state.chef.position.y as i32 - 16, fps = fps::MEDIUM);

        // Draw mice
        for mouse in &state.mice {
            sprite!("mouse", mouse.position.x as i32 - 16, mouse.position.y as i32 - 16, fps = fps::FAST);
        }

        // Draw simple crosshair
        sprite!("crosshair", state.crosshair_position.x as i32 - 16, state.crosshair_position.y as i32 - 16);
    }
    
    // render score
    text!(&format!("Score: {}", state.points), x = 20, y = 5, font = Font::L, color = 0xd14cdaff);
    
    // render lives
    text!(&format!("Lives: {}", state.chef.lives), x = 180, y = 5, font = Font::L, color = 0xd14cdaff);

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

impl Chef {
    fn move_chef(&mut self) {
        if rand() % 60*4 == 0 {
            self.direction_reverse();
        }
        
        if rand() % 60*8 == 0 {
            self.speed_increase();
        }
        
        self.position.x += self.speed;
        
        if self.position.x < 0.0 {
            self.position.x = 0.0;
            self.direction_reverse();
        } else if self.position.x > 255.0 {
            self.position.x = 255.0;
            self.direction_reverse();
        }
    }
    
    fn direction_reverse(&mut self) {
        self.speed = -self.speed;
    }
    
    fn speed_increase(&mut self) {
        if self.speed < 4.0 {
            self.speed *= 1.1;
        }
    }
}