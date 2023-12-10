use {
    crate::{
        makepad_derive_widget::*,
        widget::*,
        makepad_draw::*,
        button::{Button, ButtonAction}
    }
};

live_design!{
    LinkLabelBase = {{LinkLabel}} {}
}

#[derive(Live, LiveHook, WidgetRegister)]
pub struct LinkLabel {
    #[deref] button: Button
}

impl Widget for LinkLabel {
    fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        scope: &mut WidgetScope,
    ) {
        self.button.handle_event(cx, event, scope)
    }
    
    fn redraw(&mut self, cx: &mut Cx) {
        self.button.redraw(cx)
    }
    
    fn walk(&mut self, cx:&mut Cx) -> Walk {
        self.button.walk(cx)
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut WidgetScope, walk: Walk) -> WidgetDraw {
        self.button.draw_walk(cx, scope, walk)
    }
    
    fn text(&self)->String{
        self.button.text()
    }
    
    fn set_text(&mut self, v:&str){
        self.button.set_text(v);
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct LinkLabelRef(WidgetRef);

impl LinkLabelRef {
    pub fn clicked(&self, actions:&Actions) -> bool {
        if let Some(inner) = self.borrow(){ 
            if let Some(item) = actions.find_widget_action(inner.button.widget_uid()) {
                if let ButtonAction::Clicked = item.cast() {
                    return true
                }
            }
        }
        false
    }
    
    pub fn pressed(&self, actions:&Actions) -> bool {
        if let Some(inner) = self.borrow(){ 
            if let Some(item) = actions.find_widget_action(inner.button.widget_uid()) {
                if let ButtonAction::Pressed = item.cast() {
                    return true
                }
            }
        }
        false
    }
}
