pub struct Trade {
    entry_p: f64,
    exit_p: f64,
    is_long: bool
}

impl Trade {
    pub fn new(
        entry_p: f64,
        exit_p: f64,
        is_long: bool
    ) -> Trade {
        Trade {
            entry_p: entry_p,
            exit_p: exit_p,
            is_long: is_long
        }
    }

    pub fn get_entry_p(&self) -> f64 { self.entry_p }
    pub fn get_exit_p(&self) -> f64 { self.exit_p }

    pub fn get_side(&self) -> &str {
        if self.is_long { "long" }
        else { "short" } 
    }

    pub fn get_pnl(&self) -> f64 {
        if self.is_long { self.exit_p - self.entry_p }
        else { self.entry_p - self.exit_p }
    }

    pub fn is_win(&self) -> bool { (self.exit_p > self.entry_p && self.is_long) || (self.entry_p < self.exit_p && !self.is_long) }
}
