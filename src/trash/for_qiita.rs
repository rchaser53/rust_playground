struct Field {
    field_rows: Vec<FieldRow>
}

struct FieldRow {
    field_cells: Vec<Box<BaseCell>>
}

trait BaseCell {
    fn draw(&self) {
        // 共通処理をまず書く

        self.unique_draw();
    }
    fn unique_draw(&self) {}
}

struct ItemCell {}
impl BaseCell for ItemCell {
    fn unique_draw(&self) { /* 略 */}
}

struct WallCell {}
impl BaseCell for WallCell {
    fn unique_draw(&self) { /* 略 */}
}