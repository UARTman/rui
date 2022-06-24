use crate::*;

pub struct MapView<S1, SF, F> {
    value: S1,
    set_value: SF,
    func: F,
}

impl<S1, V, SF, F> View for MapView<S1, SF, F>
where
    V: View,
    S1: Clone + 'static,
    SF: Fn(S1, &mut Context) + 'static,
    F: Fn(State<S1>, &mut Context) -> V + 'static,
{
    fn print(&self, id: ViewId, cx: &mut Context) {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).print(id.child(&0), cx);
    }

    fn process(&self, event: &Event, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        cx.set_state(id, self.value.clone());
        let s = State::new(id);
        (self.func)(s, cx).process(event, id.child(&0), cx, vger);

        // If processing the event changed the state, then call the set_value function.
        if cx.is_dirty(id) {
            (self.set_value)(cx[s].clone(), cx)
        }
    }

    fn draw(&self, id: ViewId, cx: &mut Context, vger: &mut Vger) {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).draw(id.child(&0), cx, vger);
    }

    fn layout(&self, id: ViewId, sz: LocalSize, cx: &mut Context, vger: &mut Vger) -> LocalSize {
        cx.set_state(id, self.value.clone());

        let child_size = (self.func)(State::new(id), cx).layout(id.child(&0), sz, cx, vger);

        cx.layout.insert(
            id,
            LayoutBox {
                rect: LocalRect::new(LocalPoint::zero(), child_size),
                offset: LocalOffset::zero(),
            },
        );

        child_size
    }

    fn dirty(&self, id: ViewId, xform: LocalToWorld, cx: &mut Context) {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).dirty(id.child(&0), xform, cx);
    }

    fn hittest(
        &self,
        id: ViewId,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut Vger,
    ) -> Option<ViewId> {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).hittest(id.child(&0), pt, cx, vger)
    }

    fn commands(&self, id: ViewId, cx: &mut Context, cmds: &mut Vec<CommandInfo>) {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).commands(id.child(&0), cx, cmds);
    }

    fn gc(&self, id: ViewId, cx: &mut Context, map: &mut Vec<ViewId>) {
        cx.set_state(id, self.value.clone());
        map.push(id);
        (self.func)(State::new(id), cx).gc(id.child(&0), cx, map);
    }

    fn access(
        &self,
        id: ViewId,
        cx: &mut Context,
        nodes: &mut Vec<accesskit::Node>,
    ) -> Option<accesskit::NodeId> {
        cx.set_state(id, self.value.clone());
        (self.func)(State::new(id), cx).access(id.child(&0), cx, nodes)
    }
}

impl<S1, SF, F> private::Sealed for MapView<S1, SF, F> {}

/// Creates local derived state with a setter.
///
/// Arguments:
/// - `value` - local state value
/// - `set_value` - a function that will run each time the local state changes
/// - `func` - view function using the local state
/// 
/// Usage:
/// ```no_run
/// # use rui::*;
///
/// #[derive(Debug, Default)]
/// struct MyState {
///     x: f32,
/// }
///
/// fn main() {
///     rui(state(MyState::default, |state, cx| {
///         vstack((
///             format!("value: {:?}", cx[state]).padding(Auto),
///             map(
///                 cx[state].x * 0.01,
///                 move |v, cx| cx[state].x = v * 100.0,
///                 |s, _| knob(s).padding(Auto),
///             ),
///         ))
///     }));
/// }
/// ```
pub fn map<S, SF, F>(value: S, set_value: SF, func: F) -> impl View
where
    MapView<S, SF, F>: view::View,
{
    MapView {
        value,
        set_value,
        func,
    }
}
