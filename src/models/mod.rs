macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new(value: u64) -> Self {
                Self(value)
            }

            pub fn value(self) -> u64 {
                self.0
            }
        }
    };
}

pub mod computer;
pub mod note;
pub mod room_layout;
pub mod student;
pub mod user;

pub use computer::{Computer, ComputerAvailability, ComputerId};

pub use note::{MessageId, Note};

pub use room_layout::{ComputerPosition, RoomLayout};

pub use student::{PresenceStatus, Student, StudentId, Task, TaskStatus};

pub use user::{Role, User, UserId};
