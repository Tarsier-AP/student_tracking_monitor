pub mod computer;
pub mod student;
pub mod user;

pub use user::{Role, User, UserId};

pub use student::{PresenceStatus, Student, StudentId, Task, TaskStatus};

pub use computer::{Computer, ComputerAvailability, ComputerId};
