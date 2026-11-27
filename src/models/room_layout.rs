use super::ComputerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoomLayout {
    pub positions: Vec<ComputerPosition>,
}

impl RoomLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_position(&mut self, position: ComputerPosition) -> Result<(), String> {
        if self
            .positions
            .iter()
            .any(|p| p.computer_id == position.computer_id)
        {
            return Err(String::from(
                "Расположение этого компьютера уже существует!",
            ));
        }

        if self.cell_taken(position.computer_id, position.grid_x, position.grid_y) {
            return Err(String::from("Эта клетка уже занята другим компьютером!"));
        }

        self.positions.push(position);
        Ok(())
    }

    pub fn remove_position(&mut self, computer_id: ComputerId) -> bool {
        if let Some(index) = self
            .positions
            .iter()
            .position(|p| p.computer_id == computer_id)
        {
            self.positions.remove(index);
            return true;
        }

        false
    }

    pub fn get_position(&self, computer_id: ComputerId) -> Option<&ComputerPosition> {
        self.positions.iter().find(|p| p.computer_id == computer_id)
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
        if self.cell_taken(computer_id, grid_x, grid_y) {
            return Err(String::from("Эта клетка уже занята другим компьютером!"));
        }

        let Some(position) = self
            .positions
            .iter_mut()
            .find(|position| position.computer_id == computer_id)
        else {
            return Err(String::from("Компьютер не найден!"));
        };

        position.set_position(grid_x, grid_y);
        Ok(())
    }

    fn cell_taken(&self, computer_id: ComputerId, grid_x: u64, grid_y: u64) -> bool {
        self.positions.iter().any(|position| {
            position.computer_id != computer_id
                && position.grid_x == grid_x
                && position.grid_y == grid_y
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_position(id: u64, x: u64, y: u64) -> ComputerPosition {
        ComputerPosition::new(ComputerId(id), x, y)
    }

    #[test]
    fn computer_position_new_creates_position() {
        let position = create_position(1, 5, 10);

        assert_eq!(position.computer_id, ComputerId(1));
        assert_eq!(position.grid_x, 5);
        assert_eq!(position.grid_y, 10);
    }

    #[test]
    fn computer_position_set_position_updates_coordinates() {
        let mut position = create_position(1, 5, 10);

        position.set_position(15, 20);

        assert_eq!(position.grid_x, 15);
        assert_eq!(position.grid_y, 20);
    }

    #[test]
    fn room_layout_new_creates_empty_layout() {
        let layout = RoomLayout::new();

        assert!(layout.positions.is_empty());
    }

    #[test]
    fn room_layout_add_position_adds_position() {
        let mut layout = RoomLayout::new();

        let result = layout.add_position(create_position(1, 5, 10));

        assert!(result.is_ok());
        assert_eq!(layout.positions.len(), 1);
    }

    #[test]
    fn room_layout_add_pos_rejects_duplicate_computer_id() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let result = layout.add_position(create_position(1, 15, 20));

        assert!(result.is_err());
        assert_eq!(layout.positions.len(), 1);
    }

    #[test]
    fn room_layout_add_pos_does_not_modify_existing_pos_on_err() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let _ = layout.add_position(create_position(1, 15, 20));

        let position = layout.get_position(ComputerId(1)).unwrap();

        assert_eq!(position.grid_x, 5);
        assert_eq!(position.grid_y, 10);
    }

    #[test]
    fn room_layout_remove_pos_removes_existing_pos() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let result = layout.remove_position(ComputerId(1));

        assert!(result);
        assert!(layout.positions.is_empty());
    }

    #[test]
    fn room_layout_remove_position_returns_false_for_missing_id() {
        let mut layout = RoomLayout::new();

        let result = layout.remove_position(ComputerId(99));

        assert!(!result);
    }

    #[test]
    fn room_layout_get_position_returns_existing_position() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let position = layout.get_position(ComputerId(1));

        assert!(position.is_some());

        let position = position.unwrap();

        assert_eq!(position.grid_x, 5);
        assert_eq!(position.grid_y, 10);
    }

    #[test]
    fn room_layout_get_position_returns_none_for_missing_id() {
        let layout = RoomLayout::new();

        let position = layout.get_position(ComputerId(99));

        assert!(position.is_none());
    }

    #[test]
    fn room_layout_clear_removes_all_positions() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();
        layout.add_position(create_position(2, 15, 20)).unwrap();
        layout.add_position(create_position(3, 25, 30)).unwrap();

        layout.clear();

        assert!(layout.positions.is_empty());
    }

    #[test]
    fn room_layout_move_computer_updates_coordinates() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let result = layout.move_computer(ComputerId(1), 15, 20);

        assert!(result.is_ok());

        let position = layout.get_position(ComputerId(1)).unwrap();

        assert_eq!(position.grid_x, 15);
        assert_eq!(position.grid_y, 20);
    }

    #[test]
    fn room_layout_add_position_rejects_occupied_cell() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let result = layout.add_position(create_position(2, 5, 10));

        assert!(result.is_err());
        assert_eq!(layout.positions.len(), 1);
    }

    #[test]
    fn room_layout_move_computer_rejects_occupied_cell() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();
        layout.add_position(create_position(2, 15, 20)).unwrap();

        let result = layout.move_computer(ComputerId(2), 5, 10);

        assert!(result.is_err());

        let position = layout.get_position(ComputerId(2)).unwrap();

        assert_eq!(position.grid_x, 15);
        assert_eq!(position.grid_y, 20);
    }

    #[test]
    fn room_layout_move_computer_returns_error_for_missing_id() {
        let mut layout = RoomLayout::new();

        let result = layout.move_computer(ComputerId(99), 15, 20);

        assert!(result.is_err());
        assert!(layout.positions.is_empty());
    }

    #[test]
    fn room_layout_move_computer_does_not_modify_other_positions() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();
        layout.add_position(create_position(2, 15, 20)).unwrap();

        layout.move_computer(ComputerId(1), 50, 60).unwrap();

        let other = layout.get_position(ComputerId(2)).unwrap();

        assert_eq!(other.grid_x, 15);
        assert_eq!(other.grid_y, 20);
    }

    #[test]
    fn room_layout_move_computer_can_move_to_same_coordinates() {
        let mut layout = RoomLayout::new();

        layout.add_position(create_position(1, 5, 10)).unwrap();

        let result = layout.move_computer(ComputerId(1), 5, 10);

        assert!(result.is_ok());

        let position = layout.get_position(ComputerId(1)).unwrap();

        assert_eq!(position.grid_x, 5);
        assert_eq!(position.grid_y, 10);
    }
}
