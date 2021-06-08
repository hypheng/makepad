use {
    crate::{tab_bar::TabBar, tab_bar_logic::TabId, tree::Tree},
    makepad_render::*,
    makepad_widget::*,
};

pub struct App {
    window: DesktopWindow,
    tab_bar: TabBar,
    tree: Tree,
}

impl App {
    pub fn style(cx: &mut Cx) {
        makepad_widget::set_widget_style(cx);
        TabBar::style(cx);
        Tree::style(cx)
    }

    pub fn new(cx: &mut Cx) -> Self {
        Self {
            window: DesktopWindow::new(cx),
            tab_bar: TabBar::new(cx),
            tree: Tree::new(cx),
        }
    }

    pub fn handle_app(&mut self, cx: &mut Cx, event: &mut Event) {
        self.window.handle_desktop_window(cx, event);
        self.tab_bar.handle_event(cx, event);
        // self.tree.handle_event(cx, event);
    }

    pub fn draw_app(&mut self, cx: &mut Cx) {
        if self.window.begin_desktop_window(cx, None).is_ok() {
            if self.tab_bar.begin(cx).is_ok() {
                self.tab_bar.tab(cx, TabId(0), "AAA");
                self.tab_bar.tab(cx, TabId(1), "BBB");
                self.tab_bar.tab(cx, TabId(2), "CCC");
                self.tab_bar.end(cx);
            }
            /*
            if self.tree.begin(cx).is_ok() {
                if self.tree.begin_branch(cx, NodeId(0), "A").is_ok() {
                    if self.tree.begin_branch(cx, NodeId(1), "B").is_ok() {
                        self.tree.leaf(cx, NodeId(3), "D");
                        self.tree.leaf(cx, NodeId(4), "E");
                        self.tree.end_branch();
                    }
                    if self.tree.begin_branch(cx, NodeId(2), "C").is_ok() {
                        self.tree.leaf(cx, NodeId(5), "F");
                        self.tree.leaf(cx, NodeId(6), "G");
                        self.tree.end_branch();
                    }
                    self.tree.end_branch();
                }
                self.tree.end(cx);
            }
            */
            self.window.end_desktop_window(cx);
        }
    }
}