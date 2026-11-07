//! i3 model — tiling window manager.

#[derive(Debug, Clone)]
pub struct Workspace {
    pub number: u32,
    pub name: String,
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub app: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub workspaces: Vec<Workspace>,
    pub current: u32,
}

impl Default for Model {
    fn default() -> Self { Self { workspaces: vec![], current: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_workspace(&mut self, number: u32, name: &str) {
        if !self.workspaces.iter().any(|w| w.number == number) {
            self.workspaces.push(Workspace { number, name: name.into(), windows: vec![] });
        }
    }

    pub fn open_window(&mut self, workspace: u32, id: &str, title: &str, app: &str) -> bool {
        if let Some(w) = self.workspaces.iter_mut().find(|w| w.number == workspace) {
            w.windows.push(Window { id: id.into(), title: title.into(), app: app.into() });
            true
        } else { false }
    }

    pub fn close_window(&mut self, id: &str) -> bool {
        for w in &mut self.workspaces {
            if let Some(pos) = w.windows.iter().position(|win| win.id == id) {
                w.windows.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn switch(&mut self, number: u32) -> bool {
        if self.workspaces.iter().any(|w| w.number == number) { self.current = number; true } else { false }
    }

    pub fn list(&self) -> &[Workspace] { &self.workspaces }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workspace_window() {
        let mut i = Model::new();
        i.add_workspace(1, "main");
        i.add_workspace(2, "web");
        assert!(i.open_window(1, "w1", "Terminal", "alacritty"));
        assert!(i.switch(2));
        assert!(i.close_window("w1"));
    }
}
