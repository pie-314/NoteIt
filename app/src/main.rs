use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
}

#[component]
fn App() -> Element {
    let mut notes = use_signal(|| {
        let mut map = HashMap::new();
        let note = Note {
            id: "1".to_string(),
            title: "My First Note".to_string(),
            content: "Welcome to my note app!".to_string(),
        };
        map.insert("1".to_string(), note);
        map
    });
    
    let mut selected_id = use_signal(|| "1".to_string());
    let mut new_title = use_signal(|| "".to_string());
    
    // Simple function to add a note
    let mut add_note = move || {
        let title = new_title.read().clone();
        if title.len() > 0 {
            // Just use current time as ID
            let id = format!("{}", js_sys::Date::now() as u64);
            let note = Note {
                id: id.clone(),
                title: title,
                content: "".to_string(),
            };
            notes.write().insert(id.clone(), note);
            selected_id.set(id);
            new_title.set("".to_string());
        }
    };
    
    // Delete the current note
    let mut delete_note = move || {
        let id = selected_id.read().clone();
        notes.write().remove(&id);
        // Just select the first note we can find
        if let Some(first_id) = notes.read().keys().next() {
            selected_id.set(first_id.clone());
        }
    };
    
    // Get the current note
    let current_note = notes.read().get(&selected_id.read().clone()).cloned();
    
    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: Arial;",
            
            // Left sidebar
            div {
                style: "width: 300px; background: #f0f0f0; padding: 10px;",
                
                h1 { "My Notes" }
                
                // Add new note section
                div {
                    style: "margin-bottom: 20px;",
                    input {
                        style: "width: 200px; padding: 5px;",
                        placeholder: "New note title",
                        value: "{new_title}",
                        oninput: move |e: Event<FormData>| new_title.set(e.value()),
                    }
                    br {}
                    button {
                        style: "margin-top: 5px; padding: 5px 10px;",
                        onclick: move |_| add_note(),
                        "Add Note"
                    }
                }
                
                // List of notes
                div {
                    for (id, note) in notes.read().iter() {
                        div {
                            key: "{id}",
                            style: if *id == selected_id.read().clone() {
                                "padding: 10px; margin: 5px 0; background: #d0d0d0; cursor: pointer;"
                            } else {
                                "padding: 10px; margin: 5px 0; background: white; cursor: pointer;"
                            },
                            onclick: {
                                let note_id = id.clone();
                                move |_| selected_id.set(note_id.clone())
                            },
                            strong { "{note.title}" }
                            br {}
                            span {
                                style: "font-size: 12px; color: gray;",
                                {
                                    if note.content.len() > 30 {
                                        format!("{}...", &note.content[0..30])
                                    } else {
                                        note.content.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Right side - editor
            div {
                style: "flex: 1; padding: 20px; background: white;",
                
                if let Some(note) = current_note {
                    div {
                        // Title editor
                        h2 { "Edit Note:" }
                        input {
                            style: "width: 100%; padding: 10px; font-size: 18px; margin-bottom: 10px;",
                            value: "{note.title}",
                            oninput: {
                                let note_id = selected_id.read().clone();
                                move |e: Event<FormData>| {
                                    if let Some(n) = notes.write().get_mut(&note_id) {
                                        n.title = e.value();
                                    }
                                }
                            }
                        }
                        
                        // Content editor
                        textarea {
                            style: "width: 100%; height: 400px; padding: 10px; font-size: 14px;",
                            value: "{note.content}",
                            oninput: {
                                let note_id = selected_id.read().clone();
                                move |e: Event<FormData>| {
                                    if let Some(n) = notes.write().get_mut(&note_id) {
                                        n.content = e.value();
                                    }
                                }
                            }
                        }
                        
                        br {}
                        button {
                            style: "margin-top: 10px; padding: 8px 15px; background: red; color: white; border: none;",
                            onclick: move |_| delete_note(),
                            "Delete This Note"
                        }
                    }
                } else {
                    div {
                        h2 { "No notes yet!" }
                        p { "Create a note using the form on the left." }
                    }
                }
            }
        }
    }
}
