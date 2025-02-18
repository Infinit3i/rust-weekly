use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone)]
pub struct Todo {
    pub text: String,
    pub edit: bool,
}

pub enum Msg {
    Add,
    Update(String),
    Remove(usize),
    Edit(usize),
    UpdateEdit(String),
    Toggle(usize),
    RemoveAll,
    Nothing,
}

pub struct Model {
    input: String,
    edit_input: String,
    todos: Vec<Todo>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: "".into(),
            edit_input: "".into(),
            todos: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                if !self.input.is_empty() {
                    let todo = Todo {
                        text: self.input.clone(),
                        edit: false,
                    };
                    self.todos.push(todo);
                    self.input.clear();
                }
            }
            Msg::Update(s) => {
                self.input = s;
            }
            Msg::Remove(i) => {
                if i < self.todos.len() {
                    self.todos.remove(i);
                }
            }
            Msg::RemoveAll => {
                self.todos.clear();
            }
            Msg::UpdateEdit(s) => {
                self.edit_input = s;
            }
            Msg::Edit(i) => {
                if !self.edit_input.is_empty() && i < self.todos.len() {
                    self.todos[i].text = self.edit_input.clone();
                    self.todos[i].edit = false;
                    self.edit_input.clear();
                }
            }
            Msg::Toggle(i) => {
                if let Some(todo) = self.todos.get_mut(i) {
                    todo.edit = !todo.edit;
                    if todo.edit {
                        self.edit_input = todo.text.clone();
                    }
                }
            }
            Msg::Nothing => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        // Callback for the main input.
        let on_input = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Update(input.value())
        });
        let on_keypress = link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Msg::Add
            } else {
                Msg::Nothing
            }
        });
        let on_remove_all = link.callback(|_| Msg::RemoveAll);

        // View for each Todo item.
        let view_todo = |(i, todo): (usize, &Todo)| {
            let on_remove = link.callback(move |_| Msg::Remove(i));
            let on_toggle = link.callback(move |_| Msg::Toggle(i));

            let todo_content = if todo.edit {
                // Callback for the edit input.
                let on_edit_input = link.callback(|e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    Msg::UpdateEdit(input.value())
                });
                let on_edit_keypress = {
                    let i = i;
                    link.callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" {
                            Msg::Edit(i)
                        } else {
                            Msg::Nothing
                        }
                    })
                };

                html! {
                    <input type="text"
                           value={self.edit_input.clone()}
                           oninput={on_edit_input}
                           onkeypress={on_edit_keypress} />
                }
            } else {
                html! {
                    // Double-click toggles edit mode.
                    <span ondblclick={on_toggle.clone()}>{ &todo.text }</span>
                }
            };

            html! {
                <li>
                    { todo_content }
                    <button onclick={on_remove}>{ "X" }</button>
                </li>
            }
        };

        html! {
            <div>
                <h1>{ "Todo App" }</h1>
                <input placeholder="What do you want to do?"
                       value={self.input.clone()}
                       oninput={on_input}
                       onkeypress={on_keypress} />
                <button onclick={link.callback(|_| Msg::Add)}>{ "Add Todo" }</button>
                <div>
                    <button onclick={on_remove_all}>{ "Delete all Todos!" }</button>
                </div>
                <ul>
                    { for self.todos.iter().enumerate().map(view_todo) }
                </ul>
            </div>
        }
    }
}

fn main() {
    // This call is available because the "csr" feature is enabled.
    yew::Renderer::<Model>::new().render();
}
