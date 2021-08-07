/* Enumeration For Movement Of The Motor */
#[derive(Clone, Copy)]
enum MotorDirection {
    Reverse,
    Forward
}

/* Enumeration For States Of The Motor */
#[derive(Clone, Copy)]
pub enum MotorState {
    BootUp,
    Stopped,
    Operational,
    PreOperational,
    Faulted,
    Homing
}

/* Enumeration For States Of The Motor */
pub enum Motors {
    Motor1,
    Motor2,
    Motor3,
    Motor4
}


/* Structure Of Individual Motors */
struct MotorType {
    direction:      MotorDirection,         // Direction To Move In
    state:          MotorState,             // State Of The Motor
    count:          u32                     // Pulse Count To Count To
}

const ZERO:         u32 = 0;

impl MotorType {
    pub const fn init() -> MotorType {
        return MotorType {
            direction:  MotorDirection::Reverse,
            state:      MotorState::BootUp,
            count:      ZERO
        }
    }
}

/* Structure Of Motor Control */
pub struct MotorControl {
    motor1:         MotorType,              // Motor 1 Structure
    motor2:         MotorType,              // Motor 2 Structure
    motor3:         MotorType,              // Motor 3 Structure
    motor4:         MotorType,              // Motor 4 Structure
    state:          MotorState,             // State Of The Machine
    count:          u32,                    // Pulse Count
    toggle:         bool                    // Toggle Bit To Indicate Heartbeat Of Interrupt
}

impl MotorControl {
    pub const fn init() -> MotorControl {
        return MotorControl {
            motor1:     MotorType::init(),
            motor2:     MotorType::init(),
            motor3:     MotorType::init(),
            motor4:     MotorType::init(),
            state:      MotorState::BootUp,
            count:      0,
            toggle:     false
        }
    }

    pub fn get_motor_direction(&self, motor: Motors) -> MotorDirection {
        match motor {
            Motors::Motor1 => {
                return self.motor1.direction;
            } Motors::Motor2 => {
                return self.motor2.direction;
            } Motors::Motor3 => {
                return self.motor3.direction;
            } Motors::Motor4 => {
                return self.motor4.direction;
            }
        }
    }

    pub fn set_motor_direction(&mut self, motor: Motors, motor_dir: MotorDirection) {
        match motor {
            Motors::Motor1 => {
                self.motor1.direction = motor_dir;
            } Motors::Motor2 => {
                self.motor2.direction = motor_dir;
            } Motors::Motor3 => {
                self.motor3.direction = motor_dir;
            } Motors::Motor4 => {
                self.motor4.direction = motor_dir;   
            }
        }
    }

    pub fn get_motor_state(&self, motor: Motors) -> MotorState {
        match motor {
            Motors::Motor1 => {
                return self.motor1.state;
            } Motors::Motor2 => {
                return self.motor2.state;
            } Motors::Motor3 => {
                return self.motor3.state;
            } Motors::Motor4 => {
                return self.motor4.state;
            }
        }
    }

    pub fn set_motor_state(&mut self, motor: Motors, motor_state: MotorState) {
        match motor {
            Motors::Motor1 => {
                self.motor1.state = motor_state;
            } Motors::Motor2 => {
                self.motor2.state = motor_state;
            } Motors::Motor3 => {
                self.motor3.state = motor_state;
            } Motors::Motor4 => {
                self.motor4.state = motor_state;   
            }
        }
    }

    pub fn get_motor_count(&self, motor: Motors) -> u32 {
        match motor {
            Motors::Motor1 => {
                return self.motor1.count;
            } Motors::Motor2 => {
                return self.motor2.count;
            } Motors::Motor3 => {
                return self.motor3.count;
            } Motors::Motor4 => {
                return self.motor4.count;
            }
        }
    }

    pub fn clr_motor_count(&mut self, motor: Motors) {
        match motor {
            Motors::Motor1 => {
                self.motor1.count = ZERO;
            } Motors::Motor2 => {
                self.motor2.count = ZERO;
            } Motors::Motor3 => {
                self.motor3.count = ZERO;
            } Motors::Motor4 => {
                self.motor4.count = ZERO;   
            }
        }
    }

    pub fn set_motor_count(&mut self, motor: Motors, cnt: u32) {
        match motor {
            Motors::Motor1 => {
                self.motor1.count = cnt;
            } Motors::Motor2 => {
                self.motor2.count = cnt;
            } Motors::Motor3 => {
                self.motor3.count = cnt;
            } Motors::Motor4 => {
                self.motor4.count = cnt;   
            }
        }
    }

    pub fn get_state(&self) -> MotorState {
        return self.state;
    }

    pub fn set_state(&mut self, motor_state: MotorState) {
        self.state = motor_state;
    }

    pub fn get_count(&self) -> u32 {
        return self.count;
    }

    pub fn clr_count(&mut self) {
        self.count = ZERO;
    }

    pub fn add_count(&mut self) {
        self.count += 1;
    }

    pub fn get_toggle(&self) -> bool {
        return self.toggle;
    }

    pub fn clr_toggle(&mut self) {
        self.toggle = false;
    }

    pub fn set_toggle(&mut self) {
        self.toggle = true;
    }

    pub fn check_stopped(&self, motor_state: MotorState) -> bool {
        return match motor_state {
            MotorState::Stopped => true,
            _ => false
        }
    }
}