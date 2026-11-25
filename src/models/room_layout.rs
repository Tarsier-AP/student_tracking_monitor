
use super::ComputerId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputerPosition {
    pub computer_id: ComputerId,
    pub grid_x: u64,
    pub grid_y: u64,
}

impl ComputerPosition {
    pub fn new(computer_id: ComputerId, grid_x: u64, grid_y: u64) -> Self {
        Self {
            computer_id,
            grid_x,
            grid_y,
        }
    }

    pub fn set_position(&mut self, grid_x: u64, grid_y: u64) {
        self.grid_x = grid_x;
        self.grid_y = grid_y;
    }
}

#[derive(Debug, Clone, Default)]
pub struct RoomLayout {
    pub positions: Vec<ComputerPosition>,
}

impl RoomLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_position(
        &mut self,
        position: ComputerPosition,
    ) -> Result<(), String> {
        if self.positions.iter().any(|p| p.computer_id == position.computer_id) {
            return Err(String::from(
                "Расположение этого компьютера уже существует!",
            ));
        }

        self.positions.push(position);
        Ok(())
    }

    pub fn remove_position(&mut self, computer_id: ComputerId) -> bool {
        if let Some(index) = self.positions
            .iter()
            .position(|p| p.computer_id == computer_id)
        {
            self.positions.remove(index);
            return true;
        }

        false
    }

    pub fn get_position(
        &self,
        computer_id: ComputerId,
    ) -> Option<&ComputerPosition> {
        self.positions
            .iter()
            .find(|p| p.computer_id == computer_id)
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }

    pub fn move_computer(
        &mut self,
        computer_id: ComputerId,
        grid_x: u64,
        grid_y: u64,
    ) -> Result<(), String> {
        let position = self.positions
            .iter_mut()
            .find(|position| position.computer_id == computer_id);

        if let Some(position) = position {
            position.set_position(grid_x, grid_y);
            Ok(())
        } else {
            Err(String::from("Компьютер не найден!"))
        }
    }
}
